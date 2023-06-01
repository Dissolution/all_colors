using System.Buffers;

namespace AllColors.Originals.IMPL2;

public sealed class ImageGridCell : IEquatable<ImageGridCell>
{
    public Point Position { get; }
    public ImageGridCell[] Neighbors { get; private set; }
    
    public Option<ARGB> Color { get; set; }

    internal ImageGridCell(int x, int y)
    {
        this.Position = new Point(x,y);
        this.Color = default; // none
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
        return $"{Position}: {Color}";
    }
}