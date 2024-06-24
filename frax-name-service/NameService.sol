// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

contract NameService {
    struct NameRecord {
        string name;
        address owner;
        address destination;
    }

    mapping(string => NameRecord) public nameRecords;

    event NameRegistered(string indexed name, address indexed owner);
    event NameTransferred(string indexed name, address indexed newOwner);
    event RecordSet(string indexed name, address destination);

    function registerName(string memory _name) public {
        require(nameRecords[_name].owner == address(0), "Name already taken");

        nameRecords[_name] = NameRecord({
            name: _name,
            owner: msg.sender,
            destination: msg.sender
        });

        emit NameRegistered(_name, msg.sender);
    }

    function transferName(string memory _name, address _newOwner) public {
        require(nameRecords[_name].owner == msg.sender, "Only the owner can transfer the name");
        require(_newOwner != address(0), "Invalid new owner address");

        nameRecords[_name].owner = _newOwner;

        emit NameTransferred(_name, _newOwner);
    }

    function setRecord(string memory _name, address _destination) public {
        require(nameRecords[_name].owner == msg.sender, "Only the owner can set the record");

        nameRecords[_name].destination = _destination;

        emit RecordSet(_name, _destination);
    }

    function getNameOwner(string memory _name) public view returns (address) {
        return nameRecords[_name].owner;
    }

    function getNamedestination(string memory _name) public view returns (address) {
        return nameRecords[_name].destination;
    }
}
