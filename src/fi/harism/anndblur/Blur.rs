#pragma version(1)
#pragma rs java_package_name(fi.harism.anndblur)
#pragma rs_fp_imprecise

#include "rs_types.rsh"
#include "rs_debug.rsh"

typedef struct SizeStruct {
    int width;
    int height;
} SizeStruct_t;
SizeStruct_t sizeStruct;

typedef struct RadiusStruct {
    int radius;
    int div;
    int divsum;
} RadiusStruct_t;
RadiusStruct_t radiusStruct;

uchar4* bitmap;
uchar3* rgb;
uint8_t* dv;

int initializeDv_divsum;
void initializeDv(uint8_t* out, const void* userdata, uint32_t x) {
    *out = x / initializeDv_divsum;
}

void filll(const uint8_t* unused, const void* userData, uint32_t x) {
    for (int i = 0; i <= sizeStruct.width / 2; ++i) {
        bitmap[x * (sizeStruct.width) + i].rgb = 0;
    }
}

void blurHorizontal(const uchar4* unused, const void* userData, uint32_t x, uint32_t y) {
    uint3 sum = 0, insum = 0, outsum = 0;
    uint8_t r1 = radiusStruct.radius + 1;
    uint32_t yw = y * sizeStruct.width;
    int wmax = sizeStruct.width - 1;
    
    uint16_t stackpointer = radiusStruct.radius;
    uint16_t stackstart;
    
    uint3 stack[radiusStruct.radius + radiusStruct.radius + 1];
    
    for (int i = -radiusStruct.radius; i <= radiusStruct.radius; i++) {
        uchar3 p = bitmap[yw + min(wmax, max(i, 0))].rgb;
        
        uint3 *sir = &stack[i + radiusStruct.radius];
        (*sir).r = p.r;
        (*sir).g = p.g;
        (*sir).b = p.b;
        
        sum += (*sir) * (r1 - abs(i));
        if (i > 0) {
            insum += *sir;
        } else {
            outsum += *sir;
        }
    }
    
    for (int x = 0; x < sizeStruct.width; ++x) {
        rgb[yw + x].r = dv[sum.r];
        rgb[yw + x].g = dv[sum.g];
        rgb[yw + x].b = dv[sum.b];
        
        sum -= outsum;
        
        stackstart = stackpointer - radiusStruct.radius + radiusStruct.div;
        uint3* sir = &stack[stackstart % radiusStruct.div];
        outsum -= *sir;
        
        uchar3 p = bitmap[yw + min(x + r1, wmax)].rgb;
        (*sir).r = p.r;
        (*sir).g = p.g;
        (*sir).b = p.b;
        
        insum += *sir;
        sum += insum;
        
        stackpointer = (stackpointer + 1) % radiusStruct.div;
        sir = &stack[(stackpointer) % radiusStruct.div];
        
        outsum += *sir;
        insum -= *sir;
    }
}

void blurVertical(const uchar4* unused, const void* userData, uint32_t x, uint32_t y) {
    uint3 sum = 0, insum = 0, outsum = 0;
    
    uint8_t r1 = radiusStruct.radius + 1;
    uint32_t yi;
    int hm = sizeStruct.height - 1;
    int yp = -radiusStruct.radius * sizeStruct.width;
       
    uint16_t stackpointer = radiusStruct.radius;
    uint16_t stackstart;
    
    uint3 stack[radiusStruct.radius + radiusStruct.radius + 1];
    
    for (int i = -radiusStruct.radius; i <= radiusStruct.radius; i++) {
        yi = max(0, yp) + x;
        
        uint3* sir = &stack[i + radiusStruct.radius];
        (*sir).r = rgb[yi].r;
        (*sir).g = rgb[yi].g;
        (*sir).b = rgb[yi].b;
        
        sum += (*sir) * (r1 - abs(i));
        
        if (i > 0) {
            insum += *sir;
        } else {
            outsum += *sir;
        }
        
        if (i < hm) {
            yp += sizeStruct.width;
        }
    }
    
    yi = x;
    for (int y = 0; y < sizeStruct.height; y++) {
        bitmap[yi].r = dv[sum.r];
        bitmap[yi].g = dv[sum.g];
        bitmap[yi].b = dv[sum.b];
        
        sum -= outsum;
        
        stackstart = stackpointer - radiusStruct.radius + radiusStruct.div;
        uint3* sir = &stack[stackstart % radiusStruct.div];
        outsum -= *sir;
        
        uint32_t ymin = x + min(y + r1, hm) * sizeStruct.width;
        (*sir).r = rgb[ymin].r;
        (*sir).g = rgb[ymin].g;
        (*sir).b = rgb[ymin].b;
        insum += *sir;
        sum += insum;
        
        stackpointer = (stackpointer + 1) % radiusStruct.div;
        outsum += stack[stackpointer];
        insum -= stack[stackpointer];
        
        yi += sizeStruct.width;
    }
}
