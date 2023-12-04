const fs = require('fs');
const path = require('path')


// const input = fs.readFileSync(path.join(__dirname,'./demo'), 'utf8').split('\n');
const input = fs.readFileSync(path.join(__dirname,'./input'), 'utf8').split('\n');

const numbers = ['one','two','three','four','five','six','seven','eight','nine']
const overlapAssertion = numbers.map(number => `(?=(${number}))`);
const numberReg = new RegExp(overlapAssertion.join('|'), 'g');
const partOne = (input)=>{
    let total = 0
    for (const line of input) {
        const nums = [...line].map(c => !isNaN(parseInt(c)) ?  c : undefined).filter(it => !!it)
        if (nums.length === 0 ) continue
        const calibration = nums[0] + nums[nums.length - 1]
        total += Number(calibration)
    }
   return total
}

const partTwo = (input)=>{
    let total = 0
    for (const line of input) {
        let nums = [];
         [...line].forEach((c,i) => {
            if(!isNaN(parseInt(c))) nums.push({c,i})
        });
        [...line.matchAll(numberReg)].forEach(v=>{
            const value = v.filter(it=> !!it)
            nums.push({
                i:v.index,
                c:(numbers.indexOf(value[0].toLowerCase()) + 1).toString()
            })
        })
        nums.sort((a,b)=> a.i - b.i)
        const calibration = nums[0].c + nums[nums.length - 1].c
        total += Number(calibration)
    }
    return total
}
console.log(`part one result: ${partOne(input)}`);
console.log(`part two result: ${partTwo(input)}`);