import gc
import sys

class Dugum:
    def __init__(self, ad):
        self.ad = ad
        self.diger = None
    def __del__(self):
        print(f"  {self.ad} silindi")

a = Dugum("a")
print("a'nın referans sayısı:", sys.getrefcount(a) - 1)
del a                       # sayaç sıfıra iner: hemen silinir
print("--- döngü kuruluyor")
x, y = Dugum("x"), Dugum("y")
x.diger, y.diger = y, x     # x -> y -> x
del x, y                    # sayaçlar sıfıra inmez
print("del sonrası: hiçbir şey silinmedi")
print("gc.collect() bulduğu erişilemez nesne:", gc.collect())
