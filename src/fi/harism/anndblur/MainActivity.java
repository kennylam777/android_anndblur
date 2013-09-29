
package fi.harism.anndblur;

import fi.harism.blur.R;

import android.app.Activity;
import android.os.Bundle;
import android.support.v4.widget.DrawerLayout;
import android.view.Gravity;
import android.view.View;
import android.view.Window;
import android.widget.SeekBar;

public class MainActivity extends Activity {

    private BlurLinearLayout mFooterLayout;
    private DrawerLayout mDrawerLayout;

    @Override
    @SuppressWarnings("deprecation")
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        requestWindowFeature(Window.FEATURE_NO_TITLE);
        setContentView(R.layout.main);

        findViewById(R.id.button_left).setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                mDrawerLayout.openDrawer(Gravity.LEFT);
            }
        });

        findViewById(R.id.button_right).setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                mDrawerLayout.openDrawer(Gravity.RIGHT);
            }
        });

        int maxBlur = getWindowManager().getDefaultDisplay().getWidth() / 8;
        mFooterLayout = (BlurLinearLayout)findViewById(R.id.footer_layout);
        mFooterLayout.setBlurRadius(maxBlur / 2);

        SeekBar seekBar = (SeekBar)mFooterLayout.findViewById(R.id.seekbar_blur);
        seekBar.setMax(maxBlur);
        seekBar.setProgress(maxBlur / 2);
        seekBar.setOnSeekBarChangeListener(new SeekBar.OnSeekBarChangeListener() {
            @Override
            public void onStopTrackingTouch(SeekBar seekBar) {
            }

            @Override
            public void onStartTrackingTouch(SeekBar seekBar) {
            }

            @Override
            public void onProgressChanged(SeekBar seekBar, int progress, boolean fromUser) {
                mFooterLayout.setBlurRadius(progress + 1);
            }
        });

        mDrawerLayout = (DrawerLayout)findViewById(R.id.drawer_layout);
        mDrawerLayout.setScrimColor(0x40000000);
    }

}
