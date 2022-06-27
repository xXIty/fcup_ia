
# SYSTEM INFORMATION

- Language used: c++
- Compiler version: `g++ (GCC) 12.1.0`
- Operating System used for development and testing:

```
Linux 5.16.12-arch1-1 #1 SMP PREEMPT Wed, 02 Mar 2022 12:22:51 +0000 x86_64 GNU/Linux
```

# INSTALLATION
From the root of the project execute the following command.

`$ make`

The binary will be placed in the `bin` folder.

# USAGE

To see usage run the following command:

```
$ ./bin/main --help
Allowed options:
  --help                                Produce help message
  -H [ --hidden ] N                     Add a hidden layer with N number of 
                                        neurons
  -t [ --train ] path                   Train a new model with the data set 
                                        provided. The data set must be in .csv 
                                        format.
  -l [ --learning_rate ] arg (=0.0500000007)
                                        Use the specified learning rate
  -s [ --train_stop ] arg (=0.0500000007)
                                        Stop the learning procedure when an 
                                        absolute error (difference) is achieved
  -v [ --verbose ]                      Print loss for each epoch
```

# Usefull commands


# EXAMPLES

`./bin/main -t datasets/parity.csv -H 4 -l 0.05 -s 0.05`
