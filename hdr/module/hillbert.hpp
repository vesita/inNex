#ifndef HILLBERT_H
#define HILLBERT_H

class HilbertCurve {
public:
    // 将希尔伯特索引转换为标准二维坐标 (x, y)
    static void indexToHilbert(unsigned int n, unsigned int t, unsigned int& x, unsigned int& y);
    
    // 将标准二维坐标转换为希尔伯特索引
    static unsigned int hilbertToIndex(unsigned int n, unsigned int x, unsigned int y);
};

#endif