/*
 * @lc app=leetcode.cn id=20 lang=rust
 *
 * [20] 有效的括号
 */

// @lc code=start


impl crate::Solution {
    pub fn is_valid(raw_string: String) -> bool {
        todo!()
    }
}
// @lc code=end

/*
1
2
3
4
5
6
7
8
9
0
1
2
3
4
5
6
7
8
9
0

pub fn is_valid(raw_string: String) -> bool {
    if raw_string.len().is_multiple_of(2) {
        return false;
    }

    let map: HashMap<u8, u8> = {
        let mut map = HashMap::new();
        map.insert(b')', b'(');
        map.insert(b']', b'[');
        map.insert(b'}', b'{');
        map
    };
    let mut stack = vec![];

    for char in raw_string.bytes() {
        if !map.contains_key(&char) {
            stack.push(char);
            continue;
        }

        if stack.is_empty() || stack.pop() != map.get(&char).copied() {
            return false;
        }
    }

    stack.is_empty()
}

*/
