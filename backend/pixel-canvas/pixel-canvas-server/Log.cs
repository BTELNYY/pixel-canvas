using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace pixel_canvas_server
{
    public static class Log
    {
        public static void Info(string message)
        {
            Console.WriteLine(message);
        }

        public static void Error(string message)
        {
            Utility.WriteLineColor("Error: " + message, ConsoleColor.Red);
        }

        public static void Warning(string message)
        {
            Utility.WriteLineColor("Warning: " + message, ConsoleColor.Yellow);
        }

        public static void Debug(string message)
        {
            Utility.WriteLineColor("Debug: " + message, ConsoleColor.Gray);
        }

        public static void Success(string message)
        {
            Utility.WriteLineColor("Success: " + message, ConsoleColor.Green);
        }
    }

    public class Utility
    {
        public static void WriteLineColor(string message, ConsoleColor color)
        {
            Console.ForegroundColor = color;
            Console.WriteLine(message);
            Console.ResetColor();
        }
    }
}
