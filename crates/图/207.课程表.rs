/*
 * @lc app=leetcode.cn id=207 lang=rust
 *
 * [207] 课程表
 *
 * https://leetcode.cn/problems/course-schedule/description/
 *
 * algorithms
 * Medium (56.75%)
 * Likes:    2407
 * Dislikes: 0
 * Total Accepted:    807.8K
 * Total Submissions: 1.4M
 * Testcase Example:  '2\n[[1,0]]'
 *
 * 你这个学期必须选修 numCourses 门课程，记为 0 到 numCourses - 1 。
 *
 * 在选修某些课程之前需要一些先修课程。 先修课程按数组 prerequisites 给出，其中 prerequisites[i] = [ai, bi]
 * ，表示如果要学习课程 ai 则 必须 先学习课程  bi 。
 *
 *
 * 例如，先修课程对 [0, 1] 表示：想要学习课程 0 ，你需要先完成课程 1 。
 *
 *
 * 请你判断是否可能完成所有课程的学习？如果可以，返回 true ；否则，返回 false 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：numCourses = 2, prerequisites = [[1,0]]
 * 输出：true
 * 解释：总共有 2 门课程。学习课程 1 之前，你需要完成课程 0 。这是可能的。
 *
 * 示例 2：
 *
 *
 * 输入：numCourses = 2, prerequisites = [[1,0],[0,1]]
 * 输出：false
 * 解释：总共有 2 门课程。学习课程 1 之前，你需要先完成​课程 0 ；并且学习课程 0 之前，你还应先完成课程 1 。这是不可能的。
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= numCourses <= 2000
 * 0 <= prerequisites.length <= 5000
 * prerequisites[i].length == 2
 * 0 <= ai, bi < numCourses
 * prerequisites[i] 中的所有课程对 互不相同
 *
 *
 */

struct Solution;

// @lc code=start
use std::collections::HashMap;

impl Solution {
    fn can_finish(courses_count: i32, prerequisites: Vec<Vec<i32>>) -> bool {

        let mut course_successors_map: HashMap<usize, Vec<usize>> = HashMap::new();

        for i in prerequisites {
            let course = i[1] as usize;
            let successor = i[0] as usize;
            course_successors_map.entry(course)
                .and_modify(|vec| vec.push(successor))
                .or_insert(vec![successor]);
        }

        // 0 未访问, 1 正在访问, 2 已经访问过
        let mut visit_states = vec![0; courses_count as usize];
        for &course in course_successors_map.keys() {
            if visit_states[course] == 0 &&
                has_cycle(&course_successors_map, &mut visit_states, course)
            {
                return false;
            }
        }

        true
    }
}

fn has_cycle(
    course_successors_map: &HashMap<usize, Vec<usize>>,
    visit_states: &mut Vec<i32>,
    course: usize,
) -> bool {
    visit_states[course] = 1;

    if let Some(successors) = course_successors_map.get(&course) {
        for &successor in successors {
            match visit_states[successor] {
                0 => {
                    if has_cycle(course_successors_map, visit_states, successor) {
                        return true
                    }
                }
                1 => {return true}
                2 => {continue}
                _ => {panic!()}
            }
        }
    }

    visit_states[course] = 2;
    false
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

pub fn can_finish(courses_count: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let courses_count = courses_count as usize;

    let successor_table: HashMap<usize, Vec<usize>> =
        prerequisites
            .into_iter()
            .fold(HashMap::new(), |mut table, relationship| {
                let successor = relationship[0] as usize;
                let prerequisite = relationship[1] as usize;
                table.entry(prerequisite).or_default().push(successor);
                table
            });

    // 深度优先判断是否有环
    let mut visit_states = vec![VisitState::NotVisited; courses_count];
    for course in 0..courses_count {
        if visit_states[course] == VisitState::NotVisited
            && has_cycle(course, &successor_table, &mut visit_states)
        {
            return false;
        }
    }

    true
}

fn has_cycle(
    course: usize,
    successor_graph: &HashMap<usize, Vec<usize>>,
    visit_states: &mut [VisitState],
) -> bool {
    visit_states[course] = VisitState::Visiting;

    if let Some(successors) = successor_graph.get(&course) {
        for &successor in successors {
            match visit_states[successor] {
                VisitState::NotVisited => {
                    if has_cycle(successor, successor_graph, visit_states) {
                        return true;
                    }
                }
                VisitState::Visiting => return true,
                VisitState::Visited => continue,
            }
        }
    }

    visit_states[course] = VisitState::Visited;
    false
}

*/
