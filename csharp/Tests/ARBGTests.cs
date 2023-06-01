using System.Drawing;
using System.Reflection;
using AllColors.Drawing;
using FluentAssertions;

namespace AllColors.Tests;

public class ArbgTests
{
    [Fact]
    public void ARGB_AllRGBs_Works()
    {
        var allColors = ColorSpace.AllColors();
        var set = new HashSet<ARGB>();
        foreach (var color in allColors)
        {
            set.Add(color).Should().BeTrue();
        }
    }



    [Fact]
    public void IsCompatibleWithColor()
    {
        var namedColors = typeof(Color)
            .GetProperties(BindingFlags.Public | BindingFlags.Static)
            .Where(prop => prop.PropertyType == typeof(Color))
            .Select(prop => prop.GetValue(null))
            .OfType<Color>()
            .ToList();

        foreach (Color namedColor in namedColors)
        {
            int colorArgbValue = namedColor.ToArgb();

            ARGB argb = ARGB.FromColor(namedColor);

            int argbValue = argb.GetHashCode();

            argbValue.Should().Be(colorArgbValue);
            argb.Alpha.Should().Be(namedColor.A);
            argb.Red.Should().Be(namedColor.R);
            argb.Green.Should().Be(namedColor.G);
            argb.Blue.Should().Be(namedColor.B);
        }
    }
}