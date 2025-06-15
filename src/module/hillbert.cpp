#include "module/hillbert.hpp"

#include <utility>
void HilbertCurve::indexToHilbert(unsigned int n, unsigned int t, unsigned int& x, unsigned int& y) {
    x = y = 0;
    for (unsigned int s = 1 << (n - 1); s > 0; s >>= 1) {
        unsigned char rx = (t >> 1) & 1;
        unsigned char ry = t & 1;
        
        if (ry == 0) {
            if (rx == 1) {
                x = s - 1 - x;
                y = s - 1 - y;
            }
            // 交换x和y
            std::swap(x, y);
        } else {
            if (rx == 0) {
                x = s - 1 - x;
                y = s - 1 - y;
            }
            // 偏移量计算
            x += s * rx;
            y += s * ry;
        }
        t >>= 2;
    }
}

unsigned int HilbertCurve::hilbertToIndex(unsigned int n, unsigned int x, unsigned int y) {
    unsigned int t = 0;
    for (unsigned int s = 1 << (n - 1), level = 0; s > 0; s >>= 1, level++) {
        unsigned char rx = (x & s) > 0 ? 1 : 0;
        unsigned char ry = (y & s) > 0 ? 1 : 0;
        
        t |= (rx << (level * 2 + 1)) | (ry << (level * 2));
        
        if (ry == 0) {
            // 交换x和y
            std::swap(x, y);
            if (rx == 1) {
                x = s - 1 - x;
                y = s - 1 - y;
            }
        } else {
            if (rx == 0) {
                x = s - 1 - x;
                y = s - 1 - y;
            }
            x -= s * rx;
            y -= s * ry;
        }
    }
    return t;
}
