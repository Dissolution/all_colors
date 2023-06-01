using System.Buffers;

namespace AllColors.Originals.IMPL1;

public sealed class ImageGridCell : IEquatable<ImageGridCell>
    {
        public Point Position { get; }
        public Color PixelColor { get; set; }
        public ImageGridCell[] Neighbors { get; private set; }
        public bool IsEmpty => PixelColor.IsEmpty;

        internal ImageGridCell(int x, int y)
        {
            this.Position = new Point(x,y);
            this.PixelColor = default;
            this.Neighbors = Array.Empty<ImageGridCell>();
        }
        
        public void Initialize(ImageGrid grid)
        {
            var neighbors = ArrayPool<ImageGridCell>.Shared.Rent(8);
            int n = 0;
            for (var yOffset = -1; yOffset <= 1; yOffset++)
            {
                // validate y
                int neighborY = this.Position.Y + yOffset;
                if (neighborY < 0 || neighborY >= grid.Height) continue;
                for (var xOffset = -1; xOffset <= 1; xOffset++)
                {
                    // ignore self
                    if (yOffset == 0 && xOffset == 0) continue;
                    
                    // validate x
                    int neighborX = this.Position.X + xOffset;
                    if (neighborX < 0 || neighborX >= grid.Width) continue;

                    neighbors[n++] = grid[neighborX, neighborY];
                }
            }
            this.Neighbors = neighbors[..n].ToArray();
            ArrayPool<ImageGridCell>.Shared.Return(neighbors);
        }

        public bool Equals(ImageGridCell? cell)
        {
            return cell is not null && cell.Position == this.Position;
        }

        public override bool Equals(object? obj)
        {
            return obj is ImageGridCell cell && Equals(cell);
        }
        public override int GetHashCode()
        {
            return HashCode.Combine(Position.X, Position.Y);
        }
        public override string ToString()
        {
            return $"{Position}: {PixelColor}";
        }
    }

public sealed class ImageGrid
{
    private readonly ImageGridCell[,] _cells;
    
    public int Width { get; }
    public int Height { get; }
    
    public ImageGridCell this[int x, int y] => _cells[x, y];
    public ImageGridCell this[Point pt] => _cells[pt.X, pt.Y];

    public ImageGrid(Size size)
        : this(size.Width, size.Height)
    {
        
    }
    
    public ImageGrid(int width, int height)
    {
        this.Width = width;
        this.Height = height;
        _cells = new ImageGridCell[width, height];
        for (var y = 0; y < height; y++)
        {
            for (var x = 0; x < width; x++)
            {
                _cells[x, y] = new ImageGridCell(x, y);
            }
        }
        foreach (var cell in _cells)
        {
            cell.Initialize(this);
        }
    }
}