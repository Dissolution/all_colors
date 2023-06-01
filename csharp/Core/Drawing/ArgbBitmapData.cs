namespace AllColors.Drawing;

public ref struct ArgbBitmapData
{
    private readonly Bitmap _bitmap;
    private readonly BitmapData _bitmapData;

    private Span<byte> _bytes;

    public int Stride => _bitmapData.Stride;
    public int ByteCount => _bytes.Length;
    public int Height => _bitmapData.Height;
    public int Width => _bitmap.Width;

    public ref ARGB this[int x, int y]
    {
        get
        {
            if (x < 0 || x >= Width)
                throw new ArgumentOutOfRangeException(nameof(x), x, $"X must be between {0} and {Width}");
            if (y < 0 || y >= Height)
                throw new ArgumentOutOfRangeException(nameof(y), y, $"Y must be between 0 and {Height}");
            int offset = (4 * x) + (Stride * y);
            Span<byte> slice = _bytes.Slice(offset, 4);
            return ref MemoryMarshal.AsRef<ARGB>(slice);
        }
    }
    
    public ArgbBitmapData(Bitmap bitmap)
    {
        if (bitmap.PixelFormat != PixelFormat.Format32bppArgb)
            throw new ArgumentException("Only PixelFormat.Format32bppArgb is supported", nameof(bitmap));
        _bitmap = bitmap;
        var rect = new Rectangle(0, 0, bitmap.Width, bitmap.Height);
        _bitmapData = _bitmap.LockBits(rect, ImageLockMode.ReadWrite, PixelFormat.Format32bppArgb);

        unsafe
        {
            void* scan0 = _bitmapData.Scan0.ToPointer();
            int byteCount = _bitmapData.Stride * _bitmapData.Height;
            _bytes = new Span<byte>(scan0, byteCount);
        }
    }

    public void Dispose()
    {
        _bitmap.UnlockBits(_bitmapData);
    }
}