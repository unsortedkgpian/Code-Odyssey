package p2p

import(
	"net"
	"sync"
)

type TCPTransport struct {
	listenAddress string
	listener 	  net.Listener

	mu            sync.RWMutex // RWMuxtex is help to lock due to multiple gorouitne 
	peers 		  map[net.Addr]Peer

}

func NewTCPTransport(listenAddr string) *TCPTransport{
	return &TCPTransport{
		listenAddress: listenAddr,
	}
}