
package fi.harism.anndblur;

import fi.harism.anndblur.ScriptC_Blur;
import fi.harism.anndblur.ScriptField_RadiusStruct;
import fi.harism.anndblur.ScriptField_SizeStruct;

import android.annotation.SuppressLint;
import android.graphics.Bitmap;
import android.graphics.Canvas;
import android.graphics.Matrix;
import android.graphics.Rect;
import android.support.v8.renderscript.Allocation;
import android.support.v8.renderscript.Element;
import android.support.v8.renderscript.RenderScript;
import android.support.v8.renderscript.Script.LaunchOptions;
import android.view.View;
import android.view.ViewTreeObserver;

public class BlurRenderer {

    private static final float SCALE_FACTOR = 0.25f;

    private final View mView;
    private final Canvas mCanvas;
    private final Matrix mMatrixScale;
    private final Matrix mMatrixScaleInv;
    private final Rect mRectVisibleGlobal;
    private Bitmap mBitmap;

    private final RenderScript mRS;
    private final ScriptC_Blur mScriptBlur;
    private final LaunchOptions mLaunchOptions;
    private final ScriptField_RadiusStruct.Item mRadiusStruct;
    private final ScriptField_SizeStruct.Item mSizeStruct;

    private Allocation mAllocationBitmap;
    private Allocation mAllocationRgb;
    private Allocation mAllocationDv;

    public BlurRenderer(View view) {
        mView = view;
        mCanvas = new Canvas();

        mMatrixScale = new Matrix();
        mMatrixScale.setScale(SCALE_FACTOR, SCALE_FACTOR);
        mMatrixScaleInv = new Matrix();
        mMatrixScaleInv.setScale(1f / SCALE_FACTOR, 1f / SCALE_FACTOR);

        mRectVisibleGlobal = new Rect();

        mRS = RenderScript.create(view.getContext());
        mScriptBlur = new ScriptC_Blur(mRS);
        mLaunchOptions = new LaunchOptions();

        mRadiusStruct = new ScriptField_RadiusStruct.Item();
        mSizeStruct = new ScriptField_SizeStruct.Item();
    }

    public void onAttachedToWindow() {
        mView.getViewTreeObserver().addOnPreDrawListener(onPreDrawListener);
    }

    @SuppressLint("MissingSuperCall")
    public void onDetachedFromWindow() {
        mView.getViewTreeObserver().removeOnPreDrawListener(onPreDrawListener);
    }

    public void setBlurRadius(int radius) {
        radius = Math.round(radius * SCALE_FACTOR);

        // Setup radius struct
        mRadiusStruct.radius = Math.max(1, Math.min(254, radius));
        mRadiusStruct.div = mRadiusStruct.radius + mRadiusStruct.radius + 1;
        mRadiusStruct.divsum = (mRadiusStruct.div + 1) >> 1;
        mRadiusStruct.divsum *= mRadiusStruct.divsum;

        // Allocate dv allocation
        mAllocationDv = Allocation.createSized(mRS, Element.U8(mRS), 256 * mRadiusStruct.divsum);
        mScriptBlur.set_initializeDv_divsum(mRadiusStruct.divsum);
        mScriptBlur.forEach_initializeDv(mAllocationDv);
    }

    public boolean isOffscreenCanvas(Canvas canvas) {
        return canvas == mCanvas;
    }

    public void applyBlur() {
        mAllocationBitmap.copyFrom(mBitmap);

        mScriptBlur.set_radiusStruct(mRadiusStruct);
        mScriptBlur.bind_dv(mAllocationDv);

        mScriptBlur.bind_bitmap(mAllocationBitmap);
        mScriptBlur.bind_rgb(mAllocationRgb);
        mScriptBlur.set_sizeStruct(mSizeStruct);

        mLaunchOptions.setX(0, 1);
        mLaunchOptions.setY(0, mBitmap.getHeight());
        mScriptBlur.forEach_blurHorizontal(mAllocationBitmap, mLaunchOptions);
        mLaunchOptions.setX(0, mBitmap.getWidth());
        mLaunchOptions.setY(0, 1);
        mScriptBlur.forEach_blurVertical(mAllocationBitmap, mLaunchOptions);
        mAllocationBitmap.copyTo(mBitmap);
    }

    public void drawToCanvas(Canvas canvas) {
        if (mBitmap != null) {
            canvas.drawBitmap(mBitmap, mMatrixScaleInv, null);
        }
    }

    private void drawOffscreenBitmap() {
        mView.getGlobalVisibleRect(mRectVisibleGlobal);

        int width = Math.round(mView.getWidth() * SCALE_FACTOR);
        int height = Math.round(mView.getHeight() * SCALE_FACTOR);
        width = width & ~0x03;
        width = Math.max(width, 1);
        height = Math.max(height, 1);

        if (mBitmap == null || mBitmap.getWidth() != width || mBitmap.getHeight() != height) {
            mBitmap = Bitmap.createBitmap(width, height, Bitmap.Config.ARGB_8888);
            mAllocationBitmap = Allocation.createFromBitmap(mRS, mBitmap);
            mAllocationRgb = Allocation.createSized(mRS, Element.U8_3(mRS), width * height);
            mSizeStruct.width = width;
            mSizeStruct.height = height;

            mMatrixScaleInv.setScale((float)mView.getWidth() / width, (float)mView.getHeight() / height);
        }

        int dx = -(Math.min(0, mView.getLeft()) + mRectVisibleGlobal.left);
        int dy = -(Math.min(0, mView.getTop()) + mRectVisibleGlobal.top);

        mCanvas.restoreToCount(1);
        mCanvas.setBitmap(mBitmap);
        mCanvas.setMatrix(mMatrixScale);
        mCanvas.translate(dx, dy);
        mCanvas.clipRect(mRectVisibleGlobal);
        mCanvas.save();
        mView.getRootView().draw(mCanvas);
    }

    private final ViewTreeObserver.OnPreDrawListener onPreDrawListener = new ViewTreeObserver.OnPreDrawListener() {
        @Override
        public boolean onPreDraw() {
            if (mView.getVisibility() == View.VISIBLE) {
                drawOffscreenBitmap();
            }
            return true;
        }
    };

}
