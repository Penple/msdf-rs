#include "msdfgen.h"

/**
 * <div rustbindgen replaces="msdfgen::Bitmap"></div>
 */
template<typename T>
class Bitmap {
public:
    Bitmap();
    Bitmap(int width, int height);
private:
    T *pixels;
    int w, h;
};

/**
 * <div rustbindgen replaces="msdfgen::BitmapRef"></div>
 */
template<typename T>
class BitmapRef {
public:
    BitmapRef();
    BitmapRef(int width, int height);
private:
    T *pixels;
    int w, h;
};

/**
 * <div rustbindgen replaces="msdfgen::BitmapConstRef"></div>
 */
template<typename T>
class BitmapConstRef {
public:
    BitmapConstRef();
    BitmapConstRef(int width, int height);
private:
    T *pixels;
    int w, h;
};