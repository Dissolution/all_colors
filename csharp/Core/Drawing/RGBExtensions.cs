namespace AllColors.Drawing;

public static class RGBExtensions
{
    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    private static void MinMaxRgb(out int min, out int max, int red, int green, int blue)
    {
        if (red > green)
        {
            max = red;
            min = green;
        }
        else
        {
            max = green;
            min = red;
        }
        if (blue > max)
        {
            max = blue;
        }
        else if (blue < min)
        {
            min = blue;
        }
    }
    
    public static float GetBrightness(this RGB rgb)
    {
        MinMaxRgb(out int min, out int max, rgb.Red, rgb.Green, rgb.Blue);
        return (max + min) / (byte.MaxValue * 2.0f);
    }

    public static float GetHue(this RGB rgb)
    {
        var (r, g, b) = rgb;

        if (r == g && g == b) return 0.0f;

        MinMaxRgb(out int min, out int max, r, g, b);

        float delta = max - min;
        float hue;

        if (r == max)
            hue = (g - b) / delta;
        else if (g == max)
            hue = (b - r) / delta + 2.0f;
        else
            hue = (r - g) / delta + 4.0f;

        hue *= 60.0f;
        if (hue < 0.0f)
            hue += 360.0f;

        return hue;
    }

    public static float GetSaturation(this RGB rgb)
    {
        var (r, g, b) = rgb;

        if (r == g && g == b)
            return 0f;

        MinMaxRgb(out int min, out int max, r, g, b);

        int div = max + min;
        if (div > byte.MaxValue)
            div = (byte.MaxValue * 2) - max - min;

        return (max - min) / (float)div;
    }
}