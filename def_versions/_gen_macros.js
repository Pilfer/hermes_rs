/*

Generate Rust "build_instructions" macro body from a Hermes BytecodeList.def.
This code is disgusting. I'm sorry.

Usage: node _gen_macros.js ./v95.def

*/

const fs = require('fs');

const filename = process.argv[2];
if (!filename) {
  console.error('Please provide a filename - ex: ./v95.def');
  process.exit(1);
}

const sw = ['//', '#undef', '#endif', '#ifdef'];
let def = fs.readFileSync(filename).toString().split('\\\n').map(x => x.trim()).join(';').split('  ').join('').replace(/\(([^)]+)\)/g, m => m.replace(/\s+/g, ' '));


const insns = [];
const jmps = [];

const parse_insns = (d) => {
  let macro = d.split('(')[0];
  let args = d.split('(')[1].replace(')', '').split(' ').join('').split(',');
  const name = args.shift();
  return { macro, name, args };
};

const parse_jmp = (insn, d) => {
  let { macro, name, args } = parse_insns(d);
  name = name.replace('name', insn);
  if (name.includes('Long')) {
    name = name.replace('##', '');
  }
  return { macro, name, args };
}

let lines = def.split('\n')
  .filter(line => !sw.some(s => line.includes(s)))
  .filter(x => x.trim().length > 0 && x != undefined)
  .filter(line => {
    if (line.includes('#define DEFINE_JUMP')) {
      let items = line.split(';').filter(l => !l.includes('DEFINE_JUMP_LONG_VARIANT'));
      if (!items.length) return;
      let name = items[0].replace('#define ', '').split('(')[0].trim();
      items.shift();
      items = items.map(i => i.trim());
      jmps.push({ name, items });
    }

    if (line.startsWith('DEFINE_OPCODE_')) {
      insns.push(parse_insns(line));
    }

    if (line.startsWith('DEFINE_JUMP_')) {
      let items = line.split('(');
      let insn = items[1].replace(')', '');
      let check = jmps.find(j => j.name === items[0]);
      if (check) {
        check.items.forEach(i => {
          let okie = parse_jmp(insn, i);
          insns.push(okie);
        });
      }
    }
    return line;
  }).filter(line => {
    if (line.startsWith('DEFINE_RET_TARGET')) {
      let insn = line.replace('DEFINE_RET_TARGET(', '').replace(')', '');
      insns.find(i => i.name === insn).has_ret_target = true;
    } else if (line.startsWith('OPERAND_BIGINT_ID')) {
      let tmp = line.replace('OPERAND_BIGINT_ID(', '').replace(')', '').trim().split(',');
      let insn = tmp[0];
      let idx = tmp[1];
      let t = insns.find(i => i.name === insn);
      t.args[parseInt(idx) - 1] = `BigIntID${t.args[parseInt(idx) - 1]}`;
    } else if (line.startsWith('OPERAND_FUNCTION_ID')) {
      let tmp = line.replace('OPERAND_FUNCTION_ID(', '').replace(')', '').trim().split(',');
      let insn = tmp[0];
      let idx = tmp[1];
      let t = insns.find(i => i.name === insn);
      t.args[parseInt(idx) - 1] = `FunctionID${t.args[parseInt(idx) - 1]}`;
    } else if (line.startsWith('OPERAND_STRING_ID')) {
      let tmp = line.replace('OPERAND_STRING_ID(', '').replace(')', '').trim().split(',');
      let insn = tmp[0];
      let idx = tmp[1];
      let t = insns.find(i => i.name === insn);
      t.args[parseInt(idx) - 1] = `StringID${t.args[parseInt(idx) - 1]}`;
    }
  });


const final_insns = insns.map((i, idx) => {
  let r_count = 0;
  let p_count = 0;
  let args = i.args.map(a => {
    if (a.includes('Reg')) {
      r_count++;
      return { var: `r${r_count}`, name: a };
    } else {
      p_count++;
      return { var: `p${p_count}`, name: a };
    }
  });

  let args_str = args.map(a => `${a.var}: ${a.name}`).join(', ');
  
  return `(${idx}, ${i.name}, ${args_str})`
})

console.log(`use crate::hermes;\n\nbuild_instructions!(\n  ${final_insns.join(',\n  ')}\n);`);

