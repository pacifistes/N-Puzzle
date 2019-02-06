#!/bin/zsh
if [ $# -ne 1 ] || [[ ! $1 =~ ^(3|4)$ ]] ; then
	echo "usage:\t$0 <3|4>\n
\t$0 3 (test puzzles 3x3)
\t$0 4 (test puzzles 4x4)"
	exit 1
fi

ls test/solvable3* test/solvable4* 1> /dev/null 2>&1
if [ $? -ne 0 ] ; then
	echo "need puzzle with name test/solvable3* test/solvable4*"
	echo 'tips:\n\tfor e in {00..10} ; do
		python npuzzle-gen.py 3 -s > test/solvable3_$e;
		python npuzzle-gen.py 4 -s > test/solvable4_$e;
	done'
	exit 1
fi

cargo build --release
LOG_WORDS="len\|cpu\|number\|state\|predecessor:\|g:"
COMMIT_ID="`git rev-parse master | head -c 8`"
TIME="`date +"%s"`"
FILE_NAME="time_test_$1_${TIME}:${COMMIT_ID}.log"

echo "#########################################################################" > ${FILE_NAME}
echo "# commit: ${COMMIT_ID}" >> ${FILE_NAME}
echo "#" >> ${FILE_NAME}
echo "# for e in \`ls test/solvable$1\*\` ; do" >> ${FILE_NAME}
echo "#	echo \"\\\n\$e:\" ;" >> ${FILE_NAME}
echo "#	{ TIME ./target/release/npuzzle $e } 2>&1 | grep -i \"${LOG_WORDS}\";" >> ${FILE_NAME}
echo '# done  > ${FILE_NAME}' >> ${FILE_NAME}
echo "#" >> ${FILE_NAME}
echo "#" >> ${FILE_NAME}
echo "# # cat $FILE_NAME | grep target | awk -F ' ' '{print \$3}'" >> ${FILE_NAME}
echo "#########################################################################\n\n" >> ${FILE_NAME}

for e in `ls test/solvable$1*` ; do
	echo "\n$e:" ;
	{ time ./target/release/npuzzle $e } 2>&1 | grep -i "${LOG_WORDS}";
 done >> ${FILE_NAME}

echo "${FILE_NAME} generate"