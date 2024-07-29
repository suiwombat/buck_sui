export QUICKCHECK_TESTS=1000000
iters=0
while true
do
    cargo test --all-features --lib --quiet $1
    if [ $? -ne 0 ]
    then
        exit 1
    fi
    clear && printf '\e[3J'
    ((iters=iters+1))
    echo "iterations: $iters"
done