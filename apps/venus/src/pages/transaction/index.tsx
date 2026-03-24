import Checkout from "./checkout"
import ModalOpenBill from "./modal/open-bill"
import TransactionTabs from "./tabs"

export default function TransactionPage() {
  return (
    <>
      <TransactionTabs />
      <Checkout />
      <ModalOpenBill />
      {/* <ModalTransactionSuccess />
      <ModalDetailTransaction /> */}
    </>
  )
}
