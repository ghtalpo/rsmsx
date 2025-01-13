s = """
"""

def is_reg16(r):
    return r == 'AF' or r == 'BC' or r == 'DE' or r == 'HL' or r == 'AF_' or r == 'BC_' or r == 'DE_' or r == 'HL_' or r== 'SP'

def is_reg8(r):
    return r == 'A'or r=='F' or r == 'B'or r=='C' or r == 'D'or r=='E' or r == 'H'or r=='L'

ops = ['CALL', 'LD', 'XOR', 'OR', 'CP', 'PUSH', 'POP', 'INC', 'DEC', 'JP', 'ADD', 'ADC', 'SUB', 'SBC', 'AND', 'EX', 'RET', 'OUT', 'BIT', 'RES', 'DJNZ']
sops = ['RLA','RRA','DI','EI']
def convert_to_lua(line):
    import re
    # addr = re.compile(r"([\da-f]+)")
    addr = re.compile(r"([\da-fA-F]{4})$")
    number = re.compile(r"(0x[\da-fA-F]|\d+)")
    for op in sops:
        if op in line:
            return "self.instr_hk__%s();" % (op,)
    for op in ops:
        if op+' ' in line:
            pos = line.find(op)
            opr = line[pos+len(op):].lstrip().split('  ')[0]
            if op == 'CALL':
                v_opr = addr.search(opr)
                if v_opr:
                    return "assert!(self.call_hook(0x%s));" % v_opr.group(1)
                else:
                    return "WRONG %s %s" % (op,opr)
            elif op == 'LD':
                oprr = opr.split(',')
                if is_reg16(oprr[0]):
                    if re.match(r'^[\d]', oprr[1]):
                        # return "z80_gen.Set%s(z80, %s)" % (oprr[0],oprr[1])
                        return "self.instr_hk__LD_%s_NNNN(%s);" % (oprr[0],oprr[1])
                    elif oprr[1].startswith('('):
                        tgt = oprr[1][1:].rstrip(')').rstrip()
                        v_opr = addr.search(tgt)
                        if v_opr:
                            return "self.instr_hk__LD_%s_iNNNN(0x%s);" % (oprr[0], v_opr.group(1))
                        # elif is_reg16(tgt):
                            # return "write_word(z80, 0x%s, z80_gen.%s(z80))" % (v_opr.group(1), oprr[1])
                        else:
                            return "WRONG1 %s %s" % (op,opr)
                    else:
                        v_opr = addr.search(oprr[1])
                        if v_opr:
                            return "self.instr_hk__LD_%s_NNNN(0x%s);" % (oprr[0], v_opr.group(1))
                        else:
                            return "self.instr_hk__LD_%s_NNNN(%s);" % (oprr[0], oprr[1])
                            # return "WRONG2 %s %s" % (op,opr)
                elif is_reg8(oprr[0]):
                    if re.match(r'^[\d]', oprr[1]):
                        return "self.instr_hk__LD_%s_NN(%s);" % (oprr[0],oprr[1])
                        # return "z80.%s = %s" % (oprr[0],oprr[1])
                    elif oprr[1].startswith('('):
                        tgt = oprr[1][1:].rstrip(')').rstrip()
                        if is_reg16(tgt[:2]):
                            # return "z80.%s = read_byte(z80, z80_gen.%s(z80))" % (oprr[0], tgt[:2])
                            return "self.instr_hk__LD_%s_i%s();" % (oprr[0], tgt[:2])
                        else:
                            v_opr = addr.search(tgt)
                            if v_opr:
                                # return "z80.%s = read_byte(z80, 0x%s)" % (oprr[0], v_opr.group(1))
                                return "self.instr_hk__LD_%s_iNNNN(0x%s);" % (oprr[0], v_opr.group(1))
                            else:
                                return "WRONG1a %s %s" % (op,opr)
                    else:
                        return "self.instr_hk__LD_%s_%s();" % (oprr[0],oprr[1])
                elif oprr[0].startswith('('):
                    tgt = oprr[0][1:].rstrip(')').rstrip()
                    # print('LD tgt?', tgt)
                    v_opr = addr.search(tgt)
                    if v_opr:
                        if is_reg16(oprr[1]):
                            return "self.instr_hk__LD_iNNNN_%s(0x%s);" % (oprr[1],v_opr.group(1), )
                            # return "write_word(z80, 0x%s, z80_gen.%s(z80))" % (v_opr.group(1), oprr[1])
                        elif is_reg8(oprr[1]):
                            # return "self.instr_hk__LD_i%s_%s();" % (v_opr.group(1), oprr[1])
                            return "self.instr_hk__LD_iNNNN_%s(0x%s);" % (oprr[1],v_opr.group(1), )
                            # return "write_byte(z80, 0x%s, z80.%s)" % (v_opr.group(1), oprr[1])
                        else:
                            return "WRONG %s %s" % (op,opr)
                    elif is_reg16(tgt[:2]):
                        if is_reg8(oprr[1]):
                            return "self.instr_hk__LD_i%s_%s();" % (tgt[:2], oprr[1])
                        else:
                            return "write_byte(z80, z80_gen.%s(z80), %s)" % (tgt[:2], oprr[1])
                    else:
                        return "WRONGz %s %s" % (op,opr)
                else:
                    return "WRONG %s %s" % (op,opr)
            elif op == 'XOR':
                if opr == 'A':
                    return "z80.A = 0"
                elif re.match(r'^[\d]', opr):
                    return "z80:xor(%s)" % (opr,)
                else:
                    return "WRONG %s %s" % (op,opr)
            elif op == 'OR':
                if is_reg8(opr):
                    return "self.instr_hk__%s_A_%s();" % (op,opr)
                elif re.match(r'^[\d]', opr):
                    return "self.instr_hk__%s_NN(%s);" % (op,opr)
                    # return "z80:op_or(%s)" % (opr,)
                elif opr[0] == '(' and is_reg16(opr[1:3]):
                    return "z80:op_or(read_byte(z80, z80_gen.%s(z80)))" % (opr[1:3],)
                else:
                    return "WRONG %s %s" % (op,opr)
            elif op == 'AND':
                if is_reg8(opr):
                    return "self.instr_hk__%s_A_%s();" % (op,opr)
                elif re.match(r'^[\d]', opr):
                    return "self.instr_hk__%s_NN(%s);" % (op,opr)
                    # return "z80:op_and(%s)" % (opr,)
                else:
                    return "WRONG %s %s" % (op,opr)
            elif op == 'CP':
                if is_reg8(opr):
                    return "z80:cp(z80.%s)" % opr
                elif opr.startswith('('):
                    tgt = opr[1:].rstrip(')').rstrip()
                    if is_reg16(tgt[:2]):
                        return "self.instr_hk__%s_i%s();" % (op,tgt[:2])
                    else:
                        return "WRONGcp %s %s" % (op,tgt)
                elif number.search(opr):
                    return "self.instr_hk__%s_NN(%s);" % (op,opr)
                    # return "z80:cp(%s)" % opr
                return "WRONG %s %s" % (op,opr)
            elif op == 'PUSH':
                if is_reg16(opr[:2]):
                    return "self.instr_hk__%s_%s();" % (op,opr[:2])
                else:
                    return "WRONG %s %s" % (op,opr)
            elif op == 'POP':
                if is_reg16(opr[:2]):
                    return "self.instr_hk__%s_%s();" % (op,opr[:2])
                else:
                    return "WRONG %s %s" % (op,opr)
            elif op == 'INC':
                if opr[0] == '(' and is_reg16(opr[1:3]):
                    return "self.instr_hk__%s_i%s();" % (op,opr[1:3])
                    # return "opcodes_map.instr_hk__%s_i%s(z80)" % (op,opr[1:3])
                else:
                    return "self.instr_hk__%s_%s();" % (op,opr)
                    # return "opcodes_map.instr_hk__%s_%s(z80)" % (op,opr)
            elif op == 'DEC':
                return "self.instr_hk__%s_%s();" % (op,opr)
                # return "opcodes_map.instr_hk__%s_%s(z80)" % (op,opr)
            elif op == 'ADD':
                oprr = opr.split(',')
                if is_reg16(oprr[0]) and is_reg16(oprr[1]):
                    return "self.instr_hk__%s_%s_%s();" % (op,oprr[0],oprr[1])
                elif oprr[0] == 'A':
                    if is_reg8(oprr[1]):
                        return "self.instr_hk__%s_A_%s();" % (op,oprr[1])
                        # return "z80:add(z80.%s)" % (oprr[1],)
                    elif oprr[1][0] == '(' and is_reg16(oprr[1][1:3]):
                        return "z80:add(read_byte(z80, z80_gen.%s(z80)))" % (oprr[1][1:3],)
                    else:
                        return "self.instr_hk__%s_A_NN(%s);" % (op,oprr[1])
                        # return "z80:add(%s)" % (oprr[1],)
                # elif is_reg8(oprr[0]):
                    # return "z80.add(z80.%s)" % (oprr[1],)
                else:
                    return "WRONG %s %s" % (op,opr)
            elif op == 'ADC':
                oprr = opr.split(',')
                if is_reg16(oprr[0]) and is_reg16(oprr[1]):
                    return "opcodes_map.instr_hk__%s_%s_%s(z80)" % (op,oprr[0],oprr[1])
                elif oprr[0] == 'A':
                    if is_reg8(oprr[1]):
                        return "z80:adc(z80.%s)" % (oprr[1],)
                    elif oprr[1][0] == '(' and is_reg16(oprr[1][1:3]):
                        return "z80:adc(read_byte(z80, z80_gen.%s(z80)))" % (oprr[1][1:3],)
                    else:
                        return "z80:adc(%s)" % (oprr[1],)
                # elif is_reg8(oprr[0]):
                    # return "z80.add(z80.%s)" % (oprr[1],)
                else:
                    return "WRONG %s %s" % (op,opr)
            elif op == 'SUB':
                oprr = opr.split(',')
                if is_reg16(oprr[0]) and is_reg16(oprr[1]):
                    return "opcodes_map.instr_hk__%s_%s_%s(z80)" % (op,oprr[0],oprr[1])
                elif len(oprr) == 2 and oprr[0] == 'A':
                    return "z80:sub(%s)" % (oprr[1],)
                elif opr[0] == '(' and is_reg16(opr[1:3]):
                    return "opcodes_map.instr_hk__%s_A_i%s(z80)" % (op,opr[1:3])
                else:
                    return "WRONG %s %s" % (op,opr)
            elif op == 'SBC':
                oprr = opr.split(',')
                if is_reg16(oprr[0]) and is_reg16(oprr[1]):
                    return "opcodes_map.instr_hk__%s_%s_%s(z80)" % (op,oprr[0],oprr[1])
                elif len(oprr) == 2 and oprr[0] == 'A':
                    if oprr[1][0] == '(' and is_reg16(oprr[1][1:3]):
                        return "opcodes_map.instr_hk__%s_A_i%s(z80)" % (op,oprr[1][1:3])
                    else:
                        return "z80:sbc(%s)" % (oprr[1],)
                elif opr[0] == '(' and is_reg16(opr[1:3]):
                    return "opcodes_map.instr_hk__%s_A_i%s(z80)" % (op,opr[1:3])
                else:
                    return "WRONG %s %s" % (op,opr)
            elif op == 'EX':
                oprr = opr.split(',')
                if is_reg16(oprr[0]) and is_reg16(oprr[1]):
                    return "self.instr_hk__%s_%s_%s();" % (op,oprr[0],oprr[1])
                elif oprr[0].startswith('(') and oprr[0].endswith(')') and is_reg16(oprr[0][1:-1]) and is_reg16(oprr[1]):
                    return "self.instr_hk__%s_i%s_%s();" % (op,oprr[0][1:-1],oprr[1])
                else:
                    return "WRONG %s %s" % (op,opr)
            elif op == 'RET':
                if opr == 'Z':
                    return "self.IncPC(1);\nif (self.data.F & FLAG_Z) != 0 {\n\tself.increase_cycles(11);return true;\n} else {\n\tself.increase_cycles(5);\n}"
                else:
                    return "WRONGr? %s %s" % (op,opr)
            elif op == 'JP':
                oprr = opr.split(',')
                if len(oprr) == 1:
                    return "self.IncPC(3);self.increase_cycles(10);JP %s;\n" % oprr[0]
                elif oprr[0] == 'Z':
                    return "self.IncPC(3);self.increase_cycles(10);\nif (self.data.F & FLAG_Z) != 0 {\n\tJP %s;\n}" % oprr[1]
                elif oprr[0] == 'NZ':
                    return "self.IncPC(3);self.increase_cycles(10);\nif (self.data.F & FLAG_Z) == 0 {\n\tJP %s;\n}" % oprr[1]
                elif oprr[0] == 'NC':
                    return "self.IncPC(3);self.increase_cycles(10);\nif (self.data.F & FLAG_C) == 0 {\n\tJP %s;\n}" % oprr[1]
                else:
                    return "WRONGjp? %s %s" % (op,opr)
            elif op == 'DJNZ':
                return "self.IncPC(2);self.decB();\nif self.data.B != 0 {\n\tself.increase_cycles(13);\n\tJP %s;\n} else {\n\tself.increase_cycles(8);break;}\n" % opr
            elif op == 'OUT':
                oprr = opr.split(',')
                if oprr[0].startswith('(') and oprr[0].endswith(')'):
                    if not is_reg16(oprr[0][1:3]):
                        return "self.instr_hk__%s_iNN_%s(%s);" % (op,oprr[1],oprr[0][1:-1])
                    else:
                        return "WRONG %s %s" % (op,opr)
                else:
                    return "WRONG %s %s" % (op,opr)
            elif op == 'BIT':
                oprr = opr.split(',')
                bitn = int(oprr[0],16)
                if is_reg8(oprr[1]):
                    return "self.instr_hk__%s_%s_%s();" % (op,bitn,oprr[1])
                else:
                    return "WRONGbit %s %s" % (op,opr)
            elif op == 'RES':
                oprr = opr.split(',')
                bitn = int(oprr[0],16)
                if is_reg8(oprr[1]):
                    return "self.instr_hk__%s_%s_%s();" % (op,bitn,oprr[1])
                else:
                    return "WRONGres %s %s" % (op,opr)
            else:
                return "WRONG %s %s" % (op,opr)
    return ""


def convert(s):
    for line in s.split('\n'):
        print('//', line)
        c = convert_to_lua(line)
        if len(c) != 0:
            print(c)
    print("true")
convert(s)
