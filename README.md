# tutorial10_2

![alt text](server2.1.png)
![alt text](client1_2.1.png)
![alt text](client2_2.1.png)
![alt text](client3_2.1.png)
main di client.rs: menggunakan tokio::select! di dalam loop secara concurrent:
membaca input user dari stdin dan mengirimnya ke server
menerima pesan dari server dan menampilkannnya

Client mengirim pesan ke server
Client1 ketik "a" dan kirim ke server
Server terima "a" dan broadcast ke semua client (termasuk client1)
Client1, Client2, Client3 mulai koneksi di waktu berbeda
Jadi mereka "melihat" broadcast yang sudah terkirim sebelumnya
Client2 dan Client3 tidak dapat melihat input "a" di sisi client mereka sendiri karena "a" adalah input dari client lain, bukan dari mereka