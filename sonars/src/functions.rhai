fn fuzz(func, d, num) {
    let res = 0.0;
    res += func.call(0.0);
    for i in 0..num.to_int(){
        res += func.call(d*num);
        res += func.call(-d*num);
    }
    res/(1.0+(num*2.0))
}

fn test_func(){
    1.0
}