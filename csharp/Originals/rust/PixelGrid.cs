using System.Linq.Expressions;

namespace AllColors.Originals.rust;

public class PixelGrid
{
    private readonly Option<ARGB>[] _pixels;
    private readonly Point[][] _neighbors;
    
    public int Width { get; }
    public int Height { get; }

    public PixelGrid(int width, int height)
    {
        this.Width = width;
        this.Height = height;
        int len = width * height;
        _pixels = new Option<ARGB>[len];
        _neighbors = new Point[len][];

        Point[] getNeighbors(int x, int y)
        {
            Span<Point> neighbors = stackalloc Point[8];
            int n = 0;
            foreach (var deltaY in new int[] {-1, 0, 1})
            {
                var newY = y + deltaY;
                if (newY <= -1 || newY >= height) continue;
                foreach (var deltaX in new int[] { -1, 0, 1 })
                {
                    if (deltaY == 0 && deltaX == 0) continue;
                    var newX = x + deltaX;
                    if (newX <= -1 || newX >= width) continue;
                    neighbors[n++] = new Point(newX, newY);
                }
            }
            return neighbors[..n].ToArray();
        }

        int i = 0;
        for (var y = 0; y < height; y++)
        {
            for (var x = 0; x < width; x++)
            {
                _pixels[i] = default; // none
                _neighbors[i] = getNeighbors(x, y);
                i++;
            }
        }
        Debug.Assert(i == len);
    }

    private int GetIndex(Point pt)
    {
        return pt.X + (pt.Y * this.Width);
    }
    private int GetIndex(int x, int y)
    {
        return x + (y * this.Width);
    }

    public Option<ARGB> GetPixel(Point pt)
    {
        int index = GetIndex(pt);
        return _pixels[index];
    }
    public Option<ARGB> GetPixel(int x, int y)
    {
        int index = GetIndex(x, y);
        return _pixels[index];
    }
    
    public void SetPixel(Point pt, Option<ARGB> color)
    {
        int index = GetIndex(pt);
        _pixels[index] = color;
    }

    public Point[] GetNeighbors(Point pt)
    {
        int index = GetIndex(pt);
        return _neighbors[index];
    }
}