namespace AllColors.Drawing;

[StructLayout(LayoutKind.Explicit, Size = 3)]
public readonly struct RGB : 
    IEquatable<RGB>, IEqualityOperators<RGB, RGB, bool>,
    IEquatable<Color>
{
    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static explicit operator Color(RGB rgb) => Color.FromArgb(rgb.Red, rgb.Green, rgb.Blue);
    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static explicit operator RGB(Color color) => new RGB(color.R, color.G, color.B);
    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static bool operator ==(RGB left, RGB right) => left.Equals(right);
    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static bool operator !=(RGB left, RGB right) => !left.Equals(right);
    
    public static RGB FromColor(Color color)
    {
        return new RGB(color.R, color.G, color.B);
    }

    public static RGB Create(int red, int green, int blue)
    {
        if (red < byte.MinValue || red > byte.MaxValue)
            throw new ArgumentOutOfRangeException(nameof(red), red, $"Red must be between {byte.MinValue} and {byte.MaxValue}");
        if (green < byte.MinValue || green > byte.MaxValue)
            throw new ArgumentOutOfRangeException(nameof(green), green, $"Green must be between {byte.MinValue} and {byte.MaxValue}");
        if (blue < byte.MinValue || blue > byte.MaxValue)
            throw new ArgumentOutOfRangeException(nameof(blue), blue, $"Blue must be between {byte.MinValue} and {byte.MaxValue}");
        return new RGB((byte)red, (byte)green, (byte)blue);
    }
    
    [FieldOffset(0)]
    public readonly byte Blue;

    [FieldOffset(1)]
    public readonly byte Green;

    [FieldOffset(2)]
    public readonly byte Red;

    public bool IsEmpty => Blue == 0 && Green == 0 && Red == 0;
    
    public RGB(byte red, byte green, byte blue)
    {
        this.Red = red;
        this.Green = green;
        this.Blue = blue;
    }
    public void Deconstruct(out byte red, out byte green, out byte blue)
    {
        red = this.Red;
        green = this.Green;
        blue = this.Blue;
    }

    public Color ToColor()
    {
        return Color.FromArgb(Red, Green, Blue);
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public double Difference(RGB rgb)
    {
        int rd = Red - rgb.Red;
        int gd = Green - rgb.Green;
        int bd = Blue - rgb.Blue;
        return Math.Sqrt((rd * rd) + (gd * gd) + (bd * bd));
    }
    
    public bool Equals(RGB rgb)
    {
        return rgb.Red == this.Red && rgb.Green == this.Green && rgb.Blue == this.Blue;
    }

    public bool Equals(Color color)
    {
        return color.R == this.Red && color.G == this.Green && color.B == this.Blue;
    }

    public override bool Equals(object? obj)
    {
        if (obj is Color color)
            return Equals(color);
        if (obj is RGB rgb)
            return Equals(rgb);
        return false;
    }

    public override int GetHashCode()
    {
        return HashCode.Combine(Red, Green, Blue);
    }

    public override string ToString()
    {
        return $"[{Red:X2}{Green:X2}{Blue:X2}]";
    }
}