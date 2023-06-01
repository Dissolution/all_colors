namespace AllColors.Drawing;

[StructLayout(LayoutKind.Explicit, Size = 4)] // 4 bytes = sizeof(Int32)
[SkipLocalsInit]
public readonly struct ARGB : 
    IEquatable<ARGB>, IEqualityOperators<ARGB, ARGB, bool>,
    IEquatable<Color>,
    IFormattable
{
    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static explicit operator ARGB(Color color) => new ARGB(color.A, color.R, color.G, color.B);
    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static explicit operator Color(ARGB argb) => Color.FromArgb(argb.Alpha, argb.Red, argb.Green, argb.Blue);

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static bool operator ==(ARGB left, ARGB right) => left.Value == right.Value;
    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static bool operator !=(ARGB left, ARGB right) => left.Value != right.Value;

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

    public static ARGB FromColor(Color color) => new ARGB(color.A, color.R, color.G, color.B);
    

    [FieldOffset(0)]
    internal readonly uint Value;

    [FieldOffset(0)]
    public readonly byte Blue;

    [FieldOffset(1)]
    public readonly byte Green;

    [FieldOffset(2)]
    public readonly byte Red;

    [FieldOffset(3)]
    public readonly byte Alpha;
    
    public ARGB(byte red, byte green, byte blue)
    {
        Blue = blue;
        Green = green;
        Red = red;
        Alpha = 0;
    }
    public ARGB(byte alpha, byte red, byte green, byte blue)
    {
        Blue = blue;
        Green = green;
        Red = red;
        Alpha = alpha;
    }
    public void Deconstruct(out byte alpha, out byte red, out byte green, out byte blue)
    {
        alpha = this.Alpha;
        red = this.Red;
        green = this.Green;
        blue = this.Blue;
    }
    public void Deconstruct(out byte red, out byte green, out byte blue)
    {
        red = this.Red;
        green = this.Green;
        blue = this.Blue;
    }

    public float GetBrightness()
    {
        MinMaxRgb(out int min, out int max, Red, Green, Blue);
        return (max + min) / (byte.MaxValue * 2.0f);
    }

    public float GetHue()
    {
        var (r, g, b) = this;

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

    public float GetSaturation()
    {
        var (r, g, b) = this;

        if (r == g && g == b)
            return 0f;

        MinMaxRgb(out int min, out int max, r, g, b);

        int div = max + min;
        if (div > byte.MaxValue)
            div = (byte.MaxValue * 2) - max - min;

        return (max - min) / (float)div;
    }

    public Color ToColor()
    {
        return Color.FromArgb(Alpha, Red, Green, Blue);
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public double Difference(ARGB argb)
    {
        int rd = this.Red - argb.Red;
        int gd = this.Green - argb.Green;
        int bd = this.Blue - argb.Blue;
        return Math.Sqrt((rd * rd) + (gd * gd) + (bd * bd));
    }

    public bool Equals(ARGB argb)
    {
        return Value == argb.Value;
    }

    public bool Equals(Color color)
    {
        return color.A == this.Alpha && 
            color.R == this.Red &&
            color.G == this.Green &&
            color.B == this.Blue;
    }

    public override bool Equals(object? obj)
    {
        if (obj is ARGB argb) return Equals(argb);
        if (obj is Color color) return Equals(color);
        return false;
    }

    public override int GetHashCode()
    {
        return (int)Value;
    }

    public string ToString(string? format, IFormatProvider? _ = default)
    {
        if (format == "X" || format == "x")
        {
            return $"[{Alpha:X2}{Red:X2}{Green:X2}{Blue:X2}]";
        }
        return ToString();
    }

    public override string ToString()
    {
        return $"({Alpha},{Red},{Green},{Blue})";
    }
}