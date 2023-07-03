global using System;
using System.Net.Sockets;
using System.Net.WebSockets;

namespace pixel_canvas_server
{
    public class Program
    {
        public static string BindIP = "127.0.0.1";
        public static int Port = 8888;

        public static void Main(string[] args)
        {
            Log.Info("Server starting...");
            Log.Info("Bind IP: " + BindIP + "; Port: " + Port.ToString());
            Server.Start();
        } 
    }
}