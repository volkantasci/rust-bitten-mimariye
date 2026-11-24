fn bol(a, b) {
    return a / b;
}
// && sağ tarafı yalnız sol taraf true ise değerlendirir
print(false && bol(1, 0) == 0);
print(true || bol(1, 0) == 0);
print(true && bol(1, 0) == 0);
