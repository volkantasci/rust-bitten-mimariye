fn kare(x) {
    return x * x;
}
let t = 0;
let i = 1;
while i <= 3 && t < 100 {
    t = t + kare(i);
    i = i + 1;
}
print(t);
