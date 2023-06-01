namespace AllColors;

public static class ColorSort
{
    public static void None(Span<Color> colors)
    {
        // Do nothing!
    }
    
    public static void Randomize(Span<Color> colors, Shuffler? shuffler = null)
    {
        shuffler ??= new Shuffler();
        shuffler.Shuffle<Color>(colors);
    }

    public static void ByHue(Span<Color> colors)
    {
        colors.Sort((x,y) => x.GetHue().CompareTo(y.GetHue()));
    }
}