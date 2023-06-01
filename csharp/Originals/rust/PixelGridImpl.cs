namespace AllColors.Originals.rust;

public class PixelGridImpl
{
    public Bitmap CreateImage(ColorSpace colorSpace, Shuffler shuffler)
    {
        var stopwatch = Stopwatch.StartNew();
        var colors = colorSpace.GetArgbs();
        var starting = colorSpace.MidPoint;
        var width = colorSpace.Width;
        var height = colorSpace.Height;
        shuffler.Shuffle<ARGB>(colors);
        var grid = new PixelGrid(width, height);
        var available = new HashSet<Point>(8192);
        
        
        // set the starting pixel
        grid.SetPixel(starting, colors[0]);
        foreach (var neighbor in grid.GetNeighbors(starting))
        {
            available.Add(neighbor);
        }
        
        // process everything else
        for (var i = 1; i < colors.Length; i++)
        {
            var color = colors[i];
            
            // report progress
            if (i % 512 == 0)
            {
                Console.WriteLine($"{(double)i/colors.Length:P2}% -- queue: {available.Count}");
            }
            
            // find the best from available
            var bestPos = available
                .AsParallel()
                .Select(pt => (CalculateFit(grid, pt, color), pt))
                .MinBy(t => t.Item1)
                .pt;

            Debug.Assert(grid.GetPixel(bestPos).IsNone());
            grid.SetPixel(bestPos, color);
            available.Remove(bestPos);
            foreach (var neighbor in grid.GetNeighbors(bestPos))
            {
                if (grid.GetPixel(neighbor).IsNone())
                    available.Add(neighbor);
            }
        }
        
        Debug.Assert(available.Count == 0);
        
        var image = new Bitmap(width, height, PixelFormat.Format24bppRgb);
        for (var y = 0; y < height; y++)
        {
            for (var x = 0; x < width; x++)
            {
                var got = grid.GetPixel(x, y).IsSome(out var color);
                Debug.Assert(got);
                image.SetPixel(x, y, color.ToColor());
            }
        }
        return image;
    }

    private static int ColorDist(ARGB first, ARGB second)
    {
        var redDist = (int)first.Red - (int)second.Red;
        var greenDist = (int)first.Green - (int)second.Green;
        var blueDist = (int)first.Blue - (int)second.Blue;
        return (redDist * redDist) + (greenDist * greenDist) + (blueDist * blueDist);
    }
    
    private static int CalculateFit(PixelGrid grid, Point pos, ARGB color)
    {
        var min = grid.GetNeighbors(pos)
            .SelectWhere(pt =>
            {
                var pixel = grid.GetPixel(pt);
                if (pixel.IsSome(out var pxColor))
                {
                    return Option<int>.Some(ColorDist(pxColor, color));
                }
                return Option<int>.None();
            })
            .Min();
        return min;
    }
}