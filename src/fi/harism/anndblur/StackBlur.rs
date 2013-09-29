/*
   Copyright 2013 Harri Smatt

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
 */

//
// Stack Blur Algorithm by Mario Klingemann <mario@quasimondo.com>
// For additional information please go and see:
// http://incubator.quasimondo.com/processing/fast_blur_deluxe.php
//

#pragma version(1)
#pragma rs java_package_name(fi.harism.anndblur)
#pragma rs_fp_imprecise

#include "rs_types.rsh"

//
// For holding current bitmap size
//
typedef struct SizeStruct {
    int width;
    int height;
} SizeStruct_t;
SizeStruct_t sizeStruct;

//
// For holding current stack blur related variables
//
typedef struct RadiusStruct {
    int radius;
    int div;
    int divsum;
} RadiusStruct_t;
RadiusStruct_t radiusStruct;

//
// Variables for dynamic allocations
//
uchar4* bitmap;
uchar3* rgb;
uint8_t* dv;

//
// For initializing divisor allocation
//
int initializeDv_divsum;
void initializeDv(uint8_t* out, const void* userdata, uint32_t x) {
    *out = x / initializeDv_divsum;
}

//
// Handles horizontal stack blur step
//
void blurHorizontal(const uchar4* unused, const void* userData, uint32_t x, uint32_t y) {
    uint3 sum = 0, insum = 0, outsum = 0;
    uint8_t r1 = radiusStruct.radius + 1;
    uint32_t yw = y * sizeStruct.width;
    int wmax = sizeStruct.width - 1;
    
    uint16_t stackstart;
    uint16_t stackpointer = radiusStruct.radius;    
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

//
// Handles vertical stack blur step
//
void blurVertical(const uchar4* unused, const void* userData, uint32_t x, uint32_t y) {
    uint3 sum = 0, insum = 0, outsum = 0;
    
    uint8_t r1 = radiusStruct.radius + 1;
    uint32_t yi;
    int hmax = sizeStruct.height - 1;
    int yp = -radiusStruct.radius * sizeStruct.width;
       
    uint16_t stackstart;
    uint16_t stackpointer = radiusStruct.radius;    
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
        
        if (i < hmax) {
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
        
        uint32_t ymin = x + min(y + r1, hmax) * sizeStruct.width;
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
