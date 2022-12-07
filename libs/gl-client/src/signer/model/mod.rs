
// This file was generated by `gengrpc` from the CLN JSON-Schema.
// Do not edit this file.
//

pub mod cln;
pub mod greenlight;

/// Variants prefixed with `Gl` are deprecated and will eventually be removed.
#[derive(Debug)]
pub enum Request {
    GlGetinfo(greenlight::GetInfoRequest),
    GlStop(greenlight::StopRequest),
    GlListPeers(greenlight::ListPeersRequest),
    GlDisconnect(greenlight::DisconnectRequest),
    GlNewAddr(greenlight::NewAddrRequest),
    GlListFunds(greenlight::ListFundsRequest),
    GlWithdraw(greenlight::WithdrawRequest),
    GlFundChannel(greenlight::FundChannelRequest),
    GlCloseChannel(greenlight::CloseChannelRequest),
    GlCreateInvoice(greenlight::InvoiceRequest),
    GlPay(greenlight::PayRequest),
    GlKeysend(greenlight::KeysendRequest),
    GlListPayments(greenlight::ListPaymentsRequest),
    GlListInvoices(greenlight::ListInvoicesRequest),
    GlConnectPeer(greenlight::ConnectRequest),
    AddGossip(cln::AddgossipRequest),
    AutoCleanInvoice(cln::AutocleaninvoiceRequest),
    CheckMessage(cln::CheckmessageRequest),
    Close(cln::CloseRequest),
    Connect(cln::ConnectRequest),
    CreateInvoice(cln::CreateinvoiceRequest),
    CreateOnion(cln::CreateonionRequest),
    Datastore(cln::DatastoreRequest),
    DelDatastore(cln::DeldatastoreRequest),
    DelExpiredInvoice(cln::DelexpiredinvoiceRequest),
    DelInvoice(cln::DelinvoiceRequest),
    Disconnect(cln::DisconnectRequest),
    Feerates(cln::FeeratesRequest),
    FundChannel(cln::FundchannelRequest),
    FundPsbt(cln::FundpsbtRequest),
    GetRoute(cln::GetrouteRequest),
    Getinfo(cln::GetinfoRequest),
    Invoice(cln::InvoiceRequest),
    KeySend(cln::KeysendRequest),
    ListChannels(cln::ListchannelsRequest),
    ListDatastore(cln::ListdatastoreRequest),
    ListForwards(cln::ListforwardsRequest),
    ListFunds(cln::ListfundsRequest),
    ListInvoices(cln::ListinvoicesRequest),
    ListNodes(cln::ListnodesRequest),
    ListPays(cln::ListpaysRequest),
    ListPeers(cln::ListpeersRequest),
    ListSendPays(cln::ListsendpaysRequest),
    ListTransactions(cln::ListtransactionsRequest),
    NewAddr(cln::NewaddrRequest),
    Pay(cln::PayRequest),
    Ping(cln::PingRequest),
    SendOnion(cln::SendonionRequest),
    SendPay(cln::SendpayRequest),
    SendPsbt(cln::SendpsbtRequest),
    SetChannel(cln::SetchannelRequest),
    SignMessage(cln::SignmessageRequest),
    SignPsbt(cln::SignpsbtRequest),
    Stop(cln::StopRequest),
    TxDiscard(cln::TxdiscardRequest),
    TxPrepare(cln::TxprepareRequest),
    TxSend(cln::TxsendRequest),
    UtxoPsbt(cln::UtxopsbtRequest),
    WaitAnyInvoice(cln::WaitanyinvoiceRequest),
    WaitInvoice(cln::WaitinvoiceRequest),
    WaitSendPay(cln::WaitsendpayRequest),
    Withdraw(cln::WithdrawRequest),
}