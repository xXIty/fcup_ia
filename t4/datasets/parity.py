#!/bin/env python3

import numpy

def backtrack(vec, vec_index):
    if vec_index >= len(vec):
        print(",".join(vec) + "," + str(vec.count('1')%2))

    else:
        vec[vec_index] = '0'
        backtrack(vec, vec_index+1)
        vec[vec_index] = '1'
        backtrack(vec, vec_index+1)

if __name__ == "__main__":
    vec = ['0','0','0','0']
    backtrack(vec,0)

