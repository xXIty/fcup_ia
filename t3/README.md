
# SYSTEM INFORMATION

- Language used: c++
- Compiler version: `g++ (GCC) 12.1.0`
- Operating System used for development and testing:

```
Linux 5.16.12-arch1-1 #1 SMP PREEMPT Wed, 02 Mar 2022 12:22:51 +0000 x86_64 GNU/Linux
Linux 5.7.0-kali1-amd64 #1 SMP Debian 5.7.6-1kali2 (2020-07-01) x86_64 GNU/Linux
```

# INSTALLATION
From the root of the project execute the following command.

`make all`

The binary will be placed in the `bin` folder.

# USAGE

To see usage run the following command:

```
$ ./bin/main -h
ID3 algorithm that can be trained with continuous and multivalued input with a multivalued output.
Usage:
  ID3 algorithm [OPTION...]

  -h, --help            Print usage
  -v, --verbose         Print the decision tree.
  -f, --train_file arg  Input file for training.
  -t, --test_file arg   Input file for testing. Only attributes (no ID) 
                        with comma separated values.
  -i, --interactive     Classify space sparated rows provided by standard 
                        input.
```

# EXAMPLES

`./bin/main -f datasets/restaurant.csv -t datasets/restaurant_test.csv -v`
