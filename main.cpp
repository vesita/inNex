#include "module/hillbert.hpp"

#include <iostream>

int main() {
    std::cout << "Hilbert Curve" << std::endl;
    HilbertCurve tar;
    int length = 127;
    for (int i = 0; i < length; i++) {
        unsigned int x, y;
        tar.indexToHilbert(length, i, x, y);
        std::cout << "index: " << i << " x: " << x << " y: " << y << std::endl;
    }
}