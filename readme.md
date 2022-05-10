# Molecular Dynamics Simulation from scratch by Cpp, Rust

reference: [分子動力学ステップ・バイ・ステップ　その1](https://qiita.com/kaityo256/items/2356fff922938ae3c87c)

## Limitation of the MD calculation build by this repository.

1. the potential which will be used to calcuate the force will be the Lennard-Jones (LJ) potential only.
1. single atom type will be simulated on the MD here.
1. 3-dimention
1. square box for simulation
1. add striction to the periodic boudary condition

## Structure of the MD caluculation and simulation build in this repository.

1. ${O(N^2)}$ の原子間相互作用計算
1. ペアリストにより 原子間相互作用の計算を ${O(N)}$にする.  
    - 原子間相互作用の計算は ${O(N)}$ だが、結局ペアリストを作る処理は ${O(N^2)}$ になり、そのままでは高速化されない
    -  book keeping 法を用いてペアリストが活かされる高速化を行う
1. メッシュ探索
1. 温度制御法を3種類実装
1. 圧力測定ルーチンの実装