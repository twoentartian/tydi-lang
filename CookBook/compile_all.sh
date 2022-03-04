#!/bin/bash

cd 1_HelloWorld || exit
sh ./compile.sh
cd ..

cd 2_Generative || exit
sh ./compile.sh
cd ..

cd 3_Template || exit
sh ./compile.sh
cd ..

cd 4_Abstract_Implement || exit
sh ./compile.sh
cd ..
