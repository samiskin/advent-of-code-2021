

const test_input = `
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
`

const real_input = `
[[[[8,1],8],[[8,1],0]],[[8,[2,4]],[0,8]]]
[[[[2,2],[7,4]],[[9,1],6]],8]
[[[3,6],[[8,7],[2,9]]],[8,[[2,3],9]]]
[[[[4,5],[1,4]],1],[[0,8],[2,[6,8]]]]
[[7,[[4,6],[0,0]]],[[4,3],5]]
[[[[8,4],4],[[4,4],[1,0]]],[[5,[5,5]],[[5,2],1]]]
[[[0,[5,8]],[1,7]],[[[5,0],[1,3]],7]]
[4,[[[6,2],[7,8]],[0,[4,4]]]]
[[[3,[5,3]],8],[[[6,8],4],[8,9]]]
[[[6,0],[9,[8,1]]],[[[9,7],3],0]]
[[9,[[9,3],[0,8]]],[[[5,3],0],[[5,6],2]]]
[[3,[[7,7],3]],[[7,[5,2]],[[6,9],0]]]
[1,[4,[3,8]]]
[[[[0,2],9],[[0,7],8]],[[5,4],[2,8]]]
[[[[1,8],9],[1,7]],[[4,[8,5]],[[6,3],[1,0]]]]
[[9,[[4,3],[3,3]]],[[[4,9],[0,9]],6]]
[7,[[8,0],[5,6]]]
[[[[3,2],1],[[4,9],6]],[[9,[1,1]],[[8,7],1]]]
[[[[5,1],[3,3]],0],[1,[[3,2],2]]]
[[7,9],[[9,9],[5,[9,5]]]]
[[[[4,3],[1,7]],[4,[9,2]]],[[6,[1,7]],[[8,0],3]]]
[[[5,[2,8]],[[1,2],7]],[[3,[0,5]],[[3,5],8]]]
[[[[2,2],6],[[2,1],7]],[[[4,6],8],7]]
[[2,[[3,0],[0,5]]],[[[3,4],[0,1]],0]]
[[[[9,9],5],[[9,9],6]],[[[4,1],2],0]]
[4,[[2,9],[6,2]]]
[[[8,6],6],7]
[[7,[8,2]],[[[5,5],6],[9,0]]]
[5,[[2,5],[[4,9],[8,6]]]]
[[4,[7,[9,6]]],7]
[[[9,[3,3]],[[3,1],[8,7]]],[[6,[3,5]],[4,1]]]
[[8,6],[8,[[0,2],[8,1]]]]
[6,[8,[[7,7],0]]]
[3,4]
[[9,[8,0]],[[[7,8],3],1]]
[5,[[3,[8,7]],[[5,0],[9,7]]]]
[[[[4,2],9],[6,[0,2]]],6]
[[4,[3,[4,9]]],[[4,[1,6]],1]]
[[[6,3],[8,8]],[5,[[9,3],[6,3]]]]
[[[9,9],[[7,1],6]],[[[1,0],[7,4]],[3,[2,0]]]]
[[[[2,5],9],[3,[6,2]]],[4,7]]
[[1,[7,8]],[[[0,1],8],[[1,1],9]]]
[[[9,[6,4]],[[9,8],[0,2]]],[[[8,9],[2,3]],[3,[8,0]]]]
[[[[6,8],2],3],[[2,2],5]]
[[[4,[8,5]],[[4,3],1]],[[[2,4],[4,4]],[[4,1],[1,7]]]]
[[[[2,6],6],[[9,2],4]],[[[9,9],[9,5]],5]]
[[[[7,5],[4,9]],4],[[[0,7],[3,6]],[[6,5],[3,0]]]]
[[[4,4],[[5,7],[8,5]]],[0,8]]
[[3,[[1,3],[7,5]]],[6,[[8,1],0]]]
[[[9,9],[5,[9,6]]],[[[4,0],[5,4]],6]]
[0,[[[9,2],4],3]]
[[[1,[8,5]],[0,[6,0]]],[[[6,5],[3,1]],[[6,2],[1,5]]]]
[[[4,0],[4,7]],6]
[1,[[[5,2],9],[[3,9],4]]]
[[[[9,6],[4,1]],4],[2,[[0,2],6]]]
[9,[[[1,5],[3,1]],1]]
[5,0]
[9,[[[7,5],[2,1]],[[2,3],[5,3]]]]
[[5,[[0,5],[9,5]]],[[[2,7],3],[[2,9],[3,5]]]]
[[[1,9],2],[[7,[1,7]],[8,[9,8]]]]
[[8,9],[[5,[9,0]],[[6,8],[5,2]]]]
[6,[[[1,3],[0,8]],4]]
[[[[9,8],[0,9]],[[8,4],[3,5]]],[[[5,0],8],[[6,8],1]]]
[[6,[[1,4],[7,0]]],[[3,4],[[2,1],[2,7]]]]
[[[5,0],[3,[4,7]]],[[9,3],[[9,4],[9,6]]]]
[[[[8,3],[8,0]],5],[[[5,5],[0,2]],[[0,1],9]]]
[[[[6,4],[1,8]],[3,[0,2]]],[8,[[8,8],5]]]
[2,[[2,1],[1,4]]]
[8,[0,[3,5]]]
[[[[0,2],3],[[4,9],[1,2]]],[[8,2],[6,[7,1]]]]
[[[0,0],9],1]
[8,[[4,1],[[1,3],9]]]
[[[8,[5,9]],9],[[[5,7],[9,0]],3]]
[[5,[2,9]],7]
[5,6]
[[[[7,5],[8,3]],[[4,3],8]],[[2,2],[[7,2],[4,2]]]]
[[[9,5],[3,[1,5]]],6]
[[[[7,4],[7,9]],[[3,1],[3,1]]],[[[6,4],[0,1]],1]]
[[3,[[7,4],9]],[[[5,8],[2,7]],[[0,4],[3,6]]]]
[[[3,[2,3]],[[6,0],[7,7]]],1]
[[2,[[8,8],[2,3]]],[5,2]]
[[[0,[5,5]],[8,1]],5]
[[3,9],[6,[[0,5],[1,7]]]]
[[[[3,0],9],[8,2]],[[[2,2],8],0]]
[[[9,6],[[5,1],[4,9]]],[[[1,1],[0,3]],[[4,9],[7,5]]]]
[[[2,[6,1]],[[5,7],[9,2]]],[[[4,2],8],9]]
[[9,[7,1]],[[4,5],[9,1]]]
[[9,[5,0]],[[1,7],[[9,6],[4,5]]]]
[[[[1,1],[8,7]],4],[[0,4],[[1,7],[3,5]]]]
[[5,[1,[8,4]]],[[[9,4],0],[1,[5,5]]]]
[[[5,[1,6]],[6,0]],[[0,[9,7]],1]]
[2,[9,[[0,3],[2,3]]]]
[3,[4,[[0,9],8]]]
[[5,6],[[[9,9],[4,0]],[7,[2,0]]]]
[[[[5,1],6],[[1,0],[7,1]]],[[6,[1,0]],[[4,2],[0,0]]]]
[[[4,[0,2]],6],[[[4,3],[8,0]],[[9,6],[1,5]]]]
[[[[5,3],[2,2]],[8,[8,3]]],[[9,1],2]]
[[3,4],[[[4,7],[2,3]],[9,[9,0]]]]
[[[5,[6,2]],[[1,5],[9,2]]],[[[7,9],3],[[6,7],[6,2]]]]
[[[5,3],9],[[2,[4,3]],[[5,3],1]]]
`


type Pair = {
    val: number | { left: Pair, right: Pair },
    depth: number,
}

let parse = (str: string, i: number = 0, depth: number = 0): { pair: Pair, next_i: number } => {
    if (str[i] != '[') {
        let end = i;
        for (; end < str.length && !isNaN(parseInt(str[end], 10)); end++) {
        }
        return {
            pair: {
                val: parseInt(str.slice(i, end)),
                depth,
            },
            next_i: end,
        }
    }

    let { pair: left, next_i: after_left } = parse(str, i + 1, depth + 1);
    let { pair: right, next_i: after_right } = parse(str, after_left + 1, depth + 1);

    let pair = { val: { left, right }, depth };
    return { pair, next_i: after_right + 1 };
}

let add_to_pair = (pair: Pair, amt: number, dir: 'left' | 'right'): Pair => {
    if (typeof pair.val === 'number') {
        return {
            ...pair,
            val: pair.val + amt
        }
    }

    return {
        ...pair,
        val: {
            ...pair.val,
            [dir == 'left' ? 'right' : 'left']: add_to_pair(pair.val[dir == 'left' ? 'right' : 'left'], amt, dir)
        }
    };
}

let explode = (pair: Pair): { carry_left: number, carry_right: number } | null => {
    if (typeof pair.val == 'number') return null;


    if (pair.depth >= 4) {
        let carry = {
            carry_left: pair.val.left.val as number,
            carry_right: pair.val.right.val as number,
        }
        pair.val = 0;
        return carry;
    }

    let explode_left = explode(pair.val.left);
    if (explode_left != null) {
        pair.val.right = add_to_pair(pair.val.right, explode_left.carry_right, 'right');
        return {
            ...explode_left,
            carry_right: 0,
        }
    } else {
        let explode_right = explode(pair.val.right);
        if (explode_right != null) {
            pair.val.left = add_to_pair(pair.val.left, explode_right.carry_left, 'left');
            return {
                ...explode_right,
                carry_left: 0,
            }
        } else {
            return null;
        }
    }
}

let split = (pair: Pair): boolean => {
    if (typeof pair.val == 'number') {
        if (pair.val >= 10) {
            pair.val = {
                left: {
                    val: Math.floor(pair.val / 2),
                    depth: pair.depth + 1,
                },
                right: {
                    val: Math.ceil(pair.val / 2),
                    depth: pair.depth + 1,
                },
            }
            return true;
        }
        return false;
    } else {
        return split(pair.val.left) || split(pair.val.right);
    }
}

let pair_to_str = (pair: Pair): string => {
    if (typeof pair.val === 'number') {
        return pair.val.toString();
    }
    return `[${pair_to_str(pair.val.left)},${pair_to_str(pair.val.right)}]`
}

let inc_depth = (pair: Pair) => {
    pair.depth += 1;
    if (typeof pair.val != 'number') {
        inc_depth(pair.val.left);
        inc_depth(pair.val.right);
    }
}

let add = (left: Pair, right: Pair): Pair => {
    inc_depth(left);
    inc_depth(right);
    return {
        val: {
            left,
            right,
        },
        depth: 0,
    }
}

let magnitude = (pair: Pair): number => {
    let mag_left = typeof pair.val.left.val === 'number' ? 3 * pair.val.left.val : 3 * magnitude(pair.val.left);
    let mag_right = typeof pair.val.right.val === 'number' ? 2 * pair.val.right.val : 2 * magnitude(pair.val.right);
    return mag_left + mag_right;
}

let p1_pair = real_input.trim().split('\n').reduce((prev, curr) => {
    let parsed_left = parse(prev).pair;
    let parsed_right = parse(curr).pair;
    let parsed = add(parsed_left, parsed_right);
    while (true) {
        if (!explode(parsed) && !split(parsed)) {
            break;
        }
    }
    return pair_to_str(parsed);
})
let p1 = magnitude(parse(p1_pair).pair);

let lines = real_input.trim().split('\n');
let p2 = 0;
for (let i = 0; i < lines.length; i++) {
    for (let j = 0; j < lines.length; j++) {
        let parsed = add(parse(lines[i]).pair, parse(lines[j]).pair);
        while (true) {
            if (!explode(parsed) && !split(parsed)) {
                break;
            }
        }
        p2 = Math.max(p2, magnitude(parsed));
    }
}

console.log(p1);
console.log(p2);
