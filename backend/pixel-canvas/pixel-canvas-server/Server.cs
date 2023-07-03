using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.Sockets;
using System.Net;
using System.Text;
using System.Threading.Tasks;

namespace pixel_canvas_server
{
    public static class Server
    {
        public static Dictionary<int, Client> clients = new Dictionary<int, Client>();
        //static Thread ConsoleThread = new(ConsoleHandler.HandleCommands);
        public static void Start()
        {
            TcpListener serverSocket = new TcpListener(IPAddress.Parse(Program.BindIP), Program.Port);
            TcpClient clientSocket;
            int counter = 0;
            serverSocket.Start();
            Log.Info("Accepting client connections.");
            //ConsoleThread.Start();
            counter = 0;
            Log.Info("Server ready.");
            while (true)
            {
                counter += 1;
                clientSocket = serverSocket.AcceptTcpClient();
                Log.Debug("Client connected. ID: " + counter.ToString());
                Client client = new Client();
                client.ClientID = counter;
                var source = new CancellationTokenSource();
                client.StartClient(clientSocket, counter, source.Token, source);
                clients.Add(counter, client);
            }
        }
    }
}
