# Molecular Dynamics Simulation from scratch in Rust

reference
1. [分子動力学ステップ・バイ・ステップ by @kaityo256](https://github.com/kaityo256/mdstep.git)
1. [tinymd by @tlhr](https://github.com/tlhr/tinymd.git)

## So far, This MD can 
1. read PDB coordinates file as the init coordinates of the target molecule
1. LJ potential as the force field
1. export kinetic energy and potential energy in regard to MD steps
1. export the 3D coorinates in the original format(which will be in PDB fotmat) in respect to MD steps
1. visualize kinetic energy, potential energy and total energy in respect to MD steps as an image using the extern library "Plotters"

## Limitations of the MD simulation We build in this repository

1. Lennard-Jones potential is the only potential here we adopt as the potential which will be used to calculate the force between atoms.
1. Every atoms are treated as the same atom on this MD caluculation.
1. 3 dimentional simulation
1. Simulation box is cube
1. Classical application of the periodic boudary condition

## The schedule of our MD simulation program development

1. 計算量 $${O(N^2)}$$ の原子間相互作用計算を実装
1. 原子間相互作用の計算量が $${O(N)}$$ になるようペアリストを実装  
    - 原子間相互作用の計算は $${O(N)}$$ だが、結局ペアリストを作る処理は $${O(N^2)}$$ になりそのままでは高速化されない
    -  book keeping 法を用いてペアリストが活かされる高速化を図る
1. 一般的な立体構造データ形式への対応(PDBなど)
    - ポテンシャルとしては、依然LJポテンシャルのみを採用し、２原子間相互作用のみを考える
    - 種類の異なる原子が羅列されたデータから MD 計算する系を作れるよう実装
    - (構造が 時間進展の最中にぶっ壊れてもいいので、とりあえず PDB のデータから MD 計算できるようにする)
1. メッシュ探索
1. 温度制御法を3種類実装
1. 圧力測定ルーチンの実装