namespace AllColors.Drawing;

public static class PixelFormatExtensions
{
    public static int BytesPerPixel(this PixelFormat pixelFormat)
    {
        switch (pixelFormat)
        {
            case PixelFormat.Format8bppIndexed:
                return 1;
            case PixelFormat.Format16bppRgb555:
            case PixelFormat.Format16bppRgb565:
            case PixelFormat.Format16bppArgb1555:
            case PixelFormat.Format16bppGrayScale:
                return 2;
            case PixelFormat.Format24bppRgb:
                return 3;
            case PixelFormat.Format32bppRgb:
            case PixelFormat.Format32bppPArgb:
            case PixelFormat.Format32bppArgb:
                return 4;
            case PixelFormat.Format48bppRgb:
                return 6;
            case PixelFormat.Format64bppPArgb:
            case PixelFormat.Format64bppArgb:
                return 8;
            default:
                throw new ArgumentOutOfRangeException(nameof(pixelFormat), pixelFormat, null);
        }
    }
}