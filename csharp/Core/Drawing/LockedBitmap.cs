namespace AllColors.Drawing;

public class LockedBitmap
{
    private Bitmap _bitmap;
    private readonly BitmapData _bitmapData;

    public Rectangle Rect => new Rectangle(0, 0, _bitmapData.Width, _bitmapData.Height);
    public PixelFormat PixelFormat => _bitmapData.PixelFormat;
    public IntPtr Scan0 => _bitmapData.Scan0;
    public int Stride => _bitmapData.Stride;
    public int Width => _bitmapData.Width;
    public int Height => _bitmapData.Height;

    public RGB this[int x, int y]
    {
        get
        {
            if (x < 0 || x >= Width)
                throw new ArgumentOutOfRangeException(nameof(x), x, $"X must be between {0} and {Width}");
            if (y < 0 || y >= Height)
                throw new ArgumentOutOfRangeException(nameof(y), y, $"Y must be between 0 and {Height}");
            int bpp = PixelFormat.BytesPerPixel();
            int offset = (bpp * x) + (Stride * y);
            Span<byte> slice = this.Bytes.Slice(offset, bpp);
            return new RGB(slice[2], slice[1], slice[1]);
        }
        set
        {
            if (x < 0 || x >= Width)
                throw new ArgumentOutOfRangeException(nameof(x), x, $"X must be between {0} and {Width}");
            if (y < 0 || y >= Height)
                throw new ArgumentOutOfRangeException(nameof(y), y, $"Y must be between 0 and {Height}");
            int bpp = PixelFormat.BytesPerPixel();
            int offset = (bpp * x) + (Stride * y);
            Span<byte> slice = this.Bytes.Slice(offset, bpp);
            slice[2] = value.Red;
            slice[1] = value.Green;
            slice[0] = value.Blue;
        }
    }
    
    public ref byte FirstByte
    {
        get
        {
            unsafe
            {
                void* bitmapPtr = _bitmapData.Scan0.ToPointer();
                return ref Unsafe.AsRef<byte>(bitmapPtr);
            }
        }
    }
    public Span<byte> Bytes
    {
        get
        {
            unsafe
            {
                return new Span<byte>(_bitmapData.Scan0.ToPointer(), ByteCount);
            }
        }
    }

    public int ByteCount => _bitmapData.Stride * _bitmapData.Height;

    public LockedBitmap(Bitmap bitmap)
    {
        _bitmap = bitmap;
        var rect = new Rectangle(0, 0, bitmap.Width, bitmap.Height);
        _bitmapData = _bitmap.LockBits(rect, ImageLockMode.ReadWrite, bitmap.PixelFormat);
    }

    public void FillBytes(Span<byte> buffer)
    {
        int byteCount = this.ByteCount;
        if (byteCount > buffer.Length)
            throw new ArgumentException($"Buffer must have a capacity of {byteCount} or more", nameof(buffer));
        Unsafe.CopyBlock(
            ref buffer.GetPinnableReference(),
            ref FirstByte,
            (uint)byteCount);
    }

    public void Dispose()
    {
        _bitmap?.UnlockBits(_bitmapData);
        _bitmap = null!;
    }
}