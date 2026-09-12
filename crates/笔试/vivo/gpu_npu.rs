/*
题目描述

现有如下场景：

打开手机相机，进行拍照过程中，有N个算法需要执行，当前由于系统策略考虑，建议这些算法优先跑在NPU或者GPU上，
且不同算法在这两个硬件上的运算速度均有不同，且NPU及GPU均有算力容量限制，
如果GPU或NPU无法完成所有算法的执行，遗留的算法将仍由CPU完成，由CPU执行的算法，将不计算效率。

我们的目标是符合NPU及GPU的算力容量限制的情况下，合理安排这些算力在GPU/NPU上执行，使这些算法的运行效率最高。

输入描述
算法参数列表包括所有算法的参数，具体如下:

[算法1算力要求，算法1 NPU运行效率，算法1 GPU运行效率]，
[算法2算力要求，算法2 NPU运行效率，算法2 GPU运行效率]，
...
[算法N算力要求，算法N NPU运行效率，算法N GPU运行效率]。

输入共n+1行：

第一行输入三个整数k,m,n，k表示NPU算力容量，m表示GPU算力容量，n是算法数量。

接下来n行表示本次拍照需要实现的算法列表，具体含义见上面描述。

0 <= k, m <= 100.

1 <= n <= 100.

输出描述
输出一个整数，表示最高的效率。
输入示例
30 15 5
5 3 2
5 1 3
14 2 4
21 5 3
2 2 3
输出示例
14
提示信息
[5,3,2] [21,5,3] [2,2,3]执行在NPU，消耗NPU 5+21+2=28的算力，获得3+5+2=10的效率，[14,2,4]执行在GPU，消耗GPU 14的算力，获得4的效率，[5,1,3]因NPU\GPU算力限制，只能执行在CPU（不计效率），总效率 = 3+5+2+4=14。

时间限制：c/c++：1s；java/go：3s；其他语言：4s。
*/

#[test]
fn main() {
    use std::io::BufRead;
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let mut lines = stdin.lines();

    let line = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|str| str.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let [npu_volume, gpu_volume, count] = [line[0], line[1], line[2]];

    let mut algos = vec![];
    for _ in 0..count {
        let line = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|str| str.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let [need, npu_value, gpu_value] = [line[0], line[1], line[2]];
        algos.push((need, npu_value, gpu_value))
    }

    println!("{:?}", algos);

    let result = max_value(npu_volume, gpu_volume, algos);
    println!("{}", result);
}

fn max_value(npu_volume: usize, gpu_volume: usize, algos: Vec<(usize, usize, usize)>) -> usize {
    /*
    01背包问题

    dp数组含义:
    dp[i][j] 表示 npu_volume 为 i, gpu_volume 为 j 时, 最大的任务效率和

    */
    let mut dp = vec![vec![0; gpu_volume + 1]; npu_volume + 1];

    for (need, npu_value, gpu_value) in algos {
        // for i in (need..npu_volume + 1).rev() {
        //     for j in (need..gpu_volume + 1).rev() {
        // 上面这种写法要求同时要求了满足两个空隙, 应该改为下面这种
        for i in (0..npu_volume + 1).rev() {
            for j in (0..gpu_volume + 1).rev() {
                let mut best = dp[i][j];
                if i >= need {
                    best = best.max(dp[i - need][j] + npu_value);
                }
                if j >= need {
                    best = best.max(dp[i][j - need] + gpu_value);
                }
                dp[i][j] = best;
            }
        }
    }
    dp[npu_volume][gpu_volume]
}
