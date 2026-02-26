/*

Generate Rust "build_instructions" macro body from a Hermes BytecodeList.def.

Usage: node gen_macros.js ./v95.def

*/

import { readFileSync } from 'fs';

const filename = process.argv[2];
if (!filename) {
    console.error('Please provide a filename - ex: ./v95.def');
    process.exit(1);
}

const sw = ['//', '#undef', '#endif', '#ifdef'];
let defs = readFileSync(filename)
      .toString()
      .split('\\\n')
      .map(x => x.trim())
      .join(';')
      .split('  ')
      .join('')
      .replace(/\(([^)]+)\)/g, m => m.replace(/\s+/g, ' '));

let dump_doc = false;
let docs = [];
const insns = [];
const jmps = [];

const parse_insns = (d) => {
    let macro = d.split('(')[0];
    let args = d.split('(')[1].replace(')', '').split(' ').join('').split(',');
    const name = args.shift();
    dump_doc = true;
    return { macro, name, args, docs };
};

const parse_jmp = (insn, d) => {
    let { macro, name, args, docs } = parse_insns(d);
    name = name.replace('name', insn);
    if (name.includes('Long')) {
      name = name.replace('##', '');
    }
    return { macro, name, args, docs };
};

/**
 * @param {string} doc
*/
const is_doc_clean = (doc) => {
    if (doc.includes('!!!!!')) { return false }
    if (doc.trim() === "///") { return false }
    return true
};

for (const line of defs.split('\n').map(x => x.trim())) {
    if (line.length === 0) {
        if (dump_doc) {
            dump_doc = false;
            docs = [];
        }
        continue;
    }

    if (line.startsWith('///')) {
        if (is_doc_clean(line)) {
            docs.push(`    ${line}`);
        }
        continue;
    }

    if (sw.some(s => line.includes(s))) { continue }

    if (line.startsWith('DEFINE_OPCODE_')) {
        insns.push(parse_insns(line));
        continue;
    }

    if (line.startsWith('#define DEFINE_JUMP')) {
        let items = line.split(';').filter(l => !l.includes('DEFINE_JUMP_LONG_VARIANT'));
        if (!items.length) continue;
        let name = items[0].replace('#define ', '').split('(')[0].trim();
        items.shift();
        items = items.map(i => i.trim());
        jmps.push({ name, items });
        continue;
    }

    if (line.startsWith('DEFINE_JUMP_')) {
        let items = line.split('(');
        let insn = items[1].replace(')', '');
        let check = jmps.find(j => j.name === items[0]);
        if (check) {
            check.items.forEach(i => {
                insns.push(parse_jmp(insn, i));
            });
        }
        continue;
    }

    if (line.startsWith('DEFINE_RET_TARGET')) {
        let insn = line.replace('DEFINE_RET_TARGET(', '').replace(')', '');
        insns.find(i => i.name === insn).has_ret_target = true;
        continue;
    }

    if (line.startsWith('OPERAND_BIGINT_ID')) {
        let tmp = line.replace('OPERAND_BIGINT_ID(', '').replace(')', '').trim().split(',');
        let insn = tmp[0];
        let idx = tmp[1];
        let t = insns.find(i => i.name === insn);
        t.args[parseInt(idx) - 1] = `BigIntID${t.args[parseInt(idx) - 1]}`;
        continue;
    }

    if (line.startsWith('OPERAND_FUNCTION_ID')) {
        let tmp = line.replace('OPERAND_FUNCTION_ID(', '').replace(')', '').trim().split(',');
        let insn = tmp[0];
        let idx = tmp[1];
        let t = insns.find(i => i.name === insn);
        t.args[parseInt(idx) - 1] = `FunctionID${t.args[parseInt(idx) - 1]}`;
        continue;
    }

    if (line.startsWith('OPERAND_STRING_ID')) {
        let tmp = line.replace('OPERAND_STRING_ID(', '').replace(')', '').trim().split(',');
        let insn = tmp[0];
        let idx = tmp[1];
        let t = insns.find(i => i.name === insn);
        t.args[parseInt(idx) - 1] = `StringID${t.args[parseInt(idx) - 1]}`;
        continue;
    }
}

const final_insns = insns.map((i, idx) => {
    let r_count = 0;
    let p_count = 0;
    let args = i.args.map(a => {
      if (a.includes('Reg')) {
          let o = { var: `r${r_count}`, name: a };
          r_count++;
          return o;
      } else {
          let o = { var: `p${p_count}`, name: a };
          p_count++;
          return o;
      }
    });

    let args_str = args.map(a => `${a.var}: ${a.name}`).join(', ');

    return `${i.docs.join('\n')}\n    (${idx}, ${i.name}, ${args_str})`
})

console.log(`use crate::hermes;\n\nbuild_instructions!(\n${final_insns.join(',\n')}\n);`);

