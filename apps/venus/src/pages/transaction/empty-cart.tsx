import { TbShoppingCartOff } from "react-icons/tb"

export default function EmptyCart() {
  return (
    <div className="flex h-full flex-col items-center justify-center text-gray-400">
      <p className="mb-2 text-lg font-bold">Keranjang Kosong</p>
      <TbShoppingCartOff className="text-5xl" />
      <p className="pt-2">Silahkan pilih produk terlebih dahulu.</p>
    </div>
  )
}
