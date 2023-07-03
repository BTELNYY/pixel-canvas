using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.Sockets;
using System.Text;
using System.Threading.Tasks;

namespace pixel_canvas_server
{
    public class Client
    {
        public int ClientID = 0;
        TcpClient clientSocket = new();
        Thread? clientThread;
        CancellationToken threadToken;
        CancellationTokenSource? tokenSource;

        public void StartClient(TcpClient inClientSocket, int clientNum, CancellationToken token, CancellationTokenSource source)
        {
            clientSocket = inClientSocket;
            ClientID = clientNum;
            threadToken = token;
            tokenSource = source;
            clientThread = new(HandleClient);
            clientThread.Start();
        }
        public NetworkStream GetStream()
        {
            return clientSocket.GetStream();
        }

        public bool IsConnected()
        {
            if (clientSocket is null)
            {
                return false;
            }
            return clientSocket.Connected;
        }

        public void DestroyClient()
        {
            if (IsConnected())
            {
                clientSocket.Close();
            }
            Server.clients.Remove(ClientID);
            clientSocket = null;
            if (tokenSource == null || clientThread == null)
            {
                Log.Error("Token source or client thread are null!");
                return;
            }
            Log.Debug("Clearing threads...");
            tokenSource.Cancel();
        }

        private void HandleClient()
        {
            byte[] bytesFrom = new byte[10025];
            int requestCount = 0;

            while (tokenSource != null && !tokenSource.IsCancellationRequested)
            {
                try
                {
                    if (!IsConnected())
                    {
                        Console.WriteLine("Client Disconnected: Socket Closed.", ConsoleColor.Red);
                        DestroyClient();
                        break;
                    }
                    else
                    {
                        requestCount = requestCount + 1;
                        NetworkStream networkStream = GetStream();
                        
                        int count = networkStream.Read(bytesFrom, 0, clientSocket.ReceiveBufferSize);
                        uint ID = BitConverter.ToUInt32(bytesFrom, 0);
                        int IDint = Convert.ToInt32(ID);
                        //Console.WriteLine($"Read data! Length: {count}, ID: {ID}");
                    }
                }
                catch (Exception ex)
                {
                    Console.WriteLine($"Disconnecting client({ClientID}) due to error. Error: " + ex.Message);
                }
            }
            return;
        }
    }
}
