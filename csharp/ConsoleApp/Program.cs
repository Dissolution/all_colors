using AllColors.Originals.IMPL1;
using AllColors.Originals.IMPL2;
using AllColors.Originals.rust;
using AllColors.Originals.SE1;
using Microsoft.VisualStudio.TestPlatform.ObjectModel.Client;

//Jay_SE1.CreateImage();

//GenerateResolutions();

var timer = Stopwatch.StartNew();

var colorSpace = ColorSpace.BestRectangle(36);
var seed = 147;
var shuffler = new Shuffler(seed);

var impl = new PixelGridImpl();

timer.Restart();
//Bitmap image = impl.CreateImage(colorSpace, shuffler);
//Bitmap image = fejesjoco_SE1.CreateImage();
Bitmap image = new Jay_SE1(colorSpace, shuffler).CreateImage();
timer.Stop();
Console.WriteLine($"Total time to create image: {timer.Elapsed}");

var filePath = $@"c:\temp\all_colors\img_{seed}_{DateTime.Now:yyyyMMdd-HHmmss}.bmp";
//Directory.CreateDirectory(@"c:\temp\all_colors\");
image.Save(filePath, ImageFormat.Bmp);
image.Dispose();
//using var image = Bitmap.FromFile(filePath);
//BitmapTester.AssertUniqueColors(image as Bitmap);

Process.Start(new ProcessStartInfo()
{
    FileName = filePath,
    UseShellExecute = true,
});

Debugger.Break();



static bool IsInteresting(int integer)
{
    if (integer <= 1) return false;
    int shift = 1;
    while (true)
    {
        int value = 1 << shift;
        if (value == integer)
            return true;
        shift += 1;
        int nextValue = 1 << shift;
        if (nextValue == integer)
            return true;
        if ((value + nextValue) == integer)
            return true;
        if (value + nextValue > integer)
            return false;
    }
}

static void GenerateResolutions()
{
    var squares = new List<(int, Size)>();
    var powTwoRectangles = new List<(int, Size)>();
    var mptRects = new List<(int, Size)>();
    var rectangles = new List<(int, Size)>();
    
    for (int depth = 1; depth <= 256; depth++)
    {
        foreach (var area in ColorSpace.Areas(depth))
        {
            //Debug.Assert((area.Height * area.Width) == (depth*depth*depth));
            
            // Square?
            if (area.Height == area.Width)
            {
                squares.Add((depth, area));
            }
            // PowTwoREct?
            if (area.Height.IsPowerOfTwo() &&
                area.Width.IsPowerOfTwo())
            {
                powTwoRectangles.Add((depth, area));
            }
            if (IsInteresting(area.Height) || IsInteresting(area.Width))
            {
                mptRects.Add((depth, area));
            }
            // oth
            rectangles.Add((depth, area));
        }
    }

    var text = new StringBuilder();
    text.AppendLine("Squares");
    foreach (var tuple in squares)
    {
        text.Append($"{tuple.Item1}: {tuple.Item2.Width}x{tuple.Item2.Height}").AppendLine();
    }
    // text.AppendLine("PowTwos");
    // foreach (var tuple in powTwoRectangles)
    // {
    //     text.Append($"{tuple.Item1}: {tuple.Item2.Width}x{tuple.Item2.Height}").AppendLine();
    // }
    text.AppendLine("Interestings");
    foreach (var tuple in mptRects)
    {
        text.Append($"{tuple.Item1}: {tuple.Item2.Width}x{tuple.Item2.Height}").AppendLine();
    }
    // text.AppendLine("Rects");
    // foreach (var tuple in rectangles)
    // {
    //     text.Append($"{tuple.Item1}: {tuple.Item2.Width}x{tuple.Item2.Height}").AppendLine();
    // }

    var str = text.ToString();
    Debugger.Break();
}