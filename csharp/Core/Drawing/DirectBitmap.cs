using CommunityToolkit.HighPerformance;

namespace AllColors.Drawing;

public sealed class DirectBitmap : IDisposable
{
    private ARGB[] _argbAlloc;
    private GCHandle _allocHandle;
    private Bitmap _bitmap;

    public Span<ARGB> ARGBSpan => new Span<ARGB>(array: _argbAlloc);
    public Span2D<ARGB> ARGBSpan2d => new Span2D<ARGB>(array: _argbAlloc, height: Height, width: Width);
    
    public int Width { get; }
    public int Height { get; }

    public ref ARGB this[int x, int y]
    {
        [MethodImpl(MethodImplOptions.AggressiveInlining)]
        get => ref _argbAlloc[x + (y * Width)];
    }

    public DirectBitmap(int width, int height)
    {
        this.Width = width;
        this.Height = height;

        // Allocate enough space for this bitmap
        _argbAlloc = new ARGB[width * height];
        _allocHandle = GCHandle.Alloc(_argbAlloc, GCHandleType.Pinned);
        
        // Create the bitmap
        _bitmap = new Bitmap(width, height,
            width * 4,
            PixelFormat.Format32bppArgb,
            _allocHandle.AddrOfPinnedObject());
    }

    public ARGB GetPixel(int x, int y)
    {
        int index = x + (y * Width);
        return _argbAlloc[index];
    }
    
    public void SetPixel(int x, int y, ARGB color)
    {
        int index = x + (y * Width);
        _argbAlloc[index] = color;
    }

    public void Save(Stream stream, ImageFormat format)
    {
        _bitmap.Save(stream, format);
    }

    public void Save(string filePath, ImageFormat format)
    {
        _bitmap.Save(filePath, format);
    }

    public void Dispose()
    {
        _bitmap.Dispose();
        _bitmap = null!;
        _allocHandle.Free();
        _argbAlloc = null!;
    }
}