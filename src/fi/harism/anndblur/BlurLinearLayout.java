
package fi.harism.anndblur;

import android.content.Context;
import android.content.res.TypedArray;
import android.graphics.Canvas;
import android.util.AttributeSet;
import android.widget.LinearLayout;

public class BlurLinearLayout extends LinearLayout {

    private BlurRenderer mBlurRenderer;

    public BlurLinearLayout(Context context, AttributeSet attrs) {
        super(context, attrs);

        mBlurRenderer = new BlurRenderer(this);

        TypedArray a = getContext().obtainStyledAttributes(attrs, R.styleable.BlurView);
        for (int i = 0; i < a.getIndexCount(); ++i) {
            int attr = a.getIndex(i);
            if (attr == R.styleable.BlurView_radius) {
                int radius = a.getDimensionPixelSize(attr, 1);
                mBlurRenderer.setBlurRadius(radius);
            }
        }
        a.recycle();
    }

    @Override
    protected void onAttachedToWindow() {
        super.onAttachedToWindow();
        mBlurRenderer.onAttachedToWindow();
    }

    @Override
    protected void onDetachedFromWindow() {
        super.onDetachedFromWindow();
        mBlurRenderer.onDetachedFromWindow();
    }

    @Override
    protected void dispatchDraw(Canvas canvas) {
        if (mBlurRenderer.isOffscreenCanvas(canvas)) {
            mBlurRenderer.applyBlur();
        } else {
            mBlurRenderer.drawToCanvas(canvas);
            super.dispatchDraw(canvas);
        }
    }

    public void setBlurRadius(int radius) {
        mBlurRenderer.setBlurRadius(radius);
        invalidate();
    }

}
