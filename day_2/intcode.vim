function ExecuteBincode(noun, verb)
    " read the file
    let file_content = readfile("../input/day_2.txt")[0]
    let cont_list = split(file_content, ",")
    
    " iterate through list
    " do str2nr(expr) to convert to number
    let bincode = []
    for num in cont_list
        let bincode = add(bincode, str2nr(num))
    endfor
    
    " assign input
    let bincode[1] = a:noun
    let bincode[2] = a:verb

    " program counter
    let pc = 0
    while 1
        let opcode = bincode[pc]
        if opcode == 1
            " add two numbers: 1, SRC1, SRC2, DST
            let src1 = bincode[pc+1]
            let src2 = bincode[pc+2]
            let dst = bincode[pc+3]
            
            " let's just assume everything is alright and all indices are in range
            let result = bincode[src1] + bincode[src2]
            let bincode[dst] = result
        elseif opcode == 2
            " Multiply two numbers: 1, SRC1, SRC2, DST
            let src1 = bincode[pc+1]
            let src2 = bincode[pc+2]
            let dst = bincode[pc+3]
            
            " let's just assume everything is alright and all indices are in range
            let result = bincode[src1] * bincode[src2]
            let bincode[dst] = result
        elseif opcode == 99
            break
        else
            echom "Invalid opcode at position " . pc
            echo bincode
        endif
    
        " increment program counter
        let pc = pc + 4
    endwhile

    return bincode[0]
endfunction

echo "===== Task 1 ====="
echom "Result: " + ExecuteBincode(12, 2)

" task 2
echo "===== Task 2 ====="
let expected = 19690720
let done = 0
for n in range(100)
    for v in range(100)
        let res = ExecuteBincode(n, v)
        if res == expected
            echom "Found matching input: " . n . v
            let done = 1
            break
        endif

        if done == 1
            break
        endif
    endfor
endfor
