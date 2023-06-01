using System.Drawing;
using AllColors.Drawing;

namespace AllColors.Tests;

public static class BitmapTester
{
    public static void AssertUniqueColors(Bitmap bitmap)
    {
        using (var locked = new RgbBitmapData(bitmap))
        {
            HashSet<RGB> colors = new(locked.ByteCount/3);
            
            for (var y = 0; y < locked.Height; y++)
            {
                for (var x = 0; x < locked.Width; x++)
                {
                    RGB color = locked[x, y];
                    Assert.DoesNotContain(color, colors);
                    bool added = colors.Add(color);
                    Assert.True(added);
                }
            }
        }
    }
}