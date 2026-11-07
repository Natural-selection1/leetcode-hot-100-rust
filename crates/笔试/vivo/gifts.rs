/*
任务列表排序
题目描述
小V派大奖活动中给用户展示的任务列表需要排序，为了精细化运营，达到更高的任务转化率，
不同用户看到的任务排列顺序不同。
每个任务由专属的业务事件驱动完成，运营在管理后台配置的任务列表为 tasks，
已经按展示位置排列，对应的事件列表为 events，两个数组的长度均为 n，
对于每个下标 i，tasks[i] 和 events[i] 表示第 i 个任务的任务 ID 和事件编码。
现在推荐系统返回业务事件推荐顺序 recommend_events，数组元素为事件编码，请你根据这个推荐顺序，
调整最终展示给用户的任务列表。如果任务对应的事件编码在 recommend_events 中没有，
则这个任务保持管理后台配置的展示位置。 请返回一个任务 ID 数组，
只需要返回最前面 10 个。

输入描述:
第一行输入一个整数数组 tasks，表示任务 ID 列表，格式为 JSON 数组。
第二行输入一个字符串数组 events，表示事件编码列表，格式为 JSON 数组。
第三行输入一个字符串数组 recommend_events，表示推荐事件顺序，格式为 JSON 数组。

输出描述
输出一个整数数组，表示重新排序后的任务 ID 列表的前 10 个元素，每个元素占一行。

输入示例
[3,1,5,9,2]
["f","g","c","d","a"]
["a","b","c","d"]
输出示例
2
5
9
3
1
提示信息
数据范围：
- 1 ≤ tasks[i] ≤ 100000
- n == tasks.length == events.length
- 1 ≤ n ≤ 1000
- 1 ≤ recommend_events.length ≤ 1000
- recommend_events 中的值在 events 中不一定存在，反之亦然
- events 中的所有值互不相同 - tasks 中的所有值互不相同
提示：使用哈希表建立事件到任务的映射，然后按推荐顺序重新排列。
*/

fn solution(id:Vec<i32>, tasks: Vec<String>, recommend: Vec<String>) {
    use std::collections::HashMap;

    let mut task_id_map = HashMap::new();
    for i in 0..id.len() {
        task_id_map.insert(tasks[i], id[i]);
    }

    // 存放新的排序顺序
    let mut result:Vec<i32> = vec![];

    for i in 0..recommend.len() {
        if result.len() >= 10 {
            break;
        }

        if task_id_map.contains_key(recommend[i]) {
            result.push(task_id_map[&recommend[i]]);
            task_id_map.remove(recommend[i]);
        }
    }

    for i in 0..tasks.len() {
        if result.len() >= 10 {
            break;
        }

        if task_id_map.contains_key(tasks[i]) {
            result.push(task_id_map[&tasks[i]]);
        }
    }

    let len = result.len();
    for i in 0..len -1 {
        println!("{}", result[i]);
    }
    print!("{}", result[len - 1]);
}